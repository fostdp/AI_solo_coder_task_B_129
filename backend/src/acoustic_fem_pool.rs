use crate::config::AppConfig;
use crate::models::{EthnicDrumProfile, SpectrumBin};
use std::sync::Arc;
use tokio::task;

pub struct AcousticFemPool {
    config: Arc<AppConfig>,
}

impl AcousticFemPool {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    fn ac(&self) -> &crate::config::AcousticsConfig {
        self.config.acoustics.as_ref().unwrap()
    }

    pub fn get_lambda(&self, m: usize, n: usize) -> f64 {
        self.ac().eigenvalues_lambda
            .iter()
            .find(|e| e.m == m && e.n == n)
            .map(|e| e.lambda)
            .unwrap_or_else(|| {
                (n as f64 + m as f64 * 0.5) * std::f64::consts::PI
            })
    }

    pub fn factorial(n: f64) -> f64 {
        if n < 0.0 {
            1.0
        } else {
            let mut result = 1.0_f64;
            let ni = n as usize;
            for i in 1..=ni {
                result *= i as f64;
            }
            result
        }
    }

    pub fn bessel_j(m: i32, x: f64) -> f64 {
        if x.abs() < 1e-10 {
            return if m == 0 { 1.0 } else { 0.0 };
        }
        let x2 = x / 2.0;
        let mut sum = 0.0_f64;
        let mut sign = 1.0_f64;
        let mut x_pow = x2.powi(m);

        for k in 0..30 {
            let k_fact = Self::factorial(k as f64);
            let mk_fact = Self::factorial((m + k) as f64);
            if mk_fact.is_infinite() || k_fact.is_infinite() || x_pow.is_infinite() {
                break;
            }
            let denom = k_fact * mk_fact;
            if denom.abs() < 1e-20 {
                break;
            }
            let term = sign * x_pow / denom;
            sum += term;
            if term.abs() < 1e-12 * sum.abs().max(1.0) {
                break;
            }
            sign *= -1.0;
            x_pow *= x2 * x2;
        }
        sum
    }

    pub fn bessel_i(m: i32, x: f64) -> f64 {
        if x.abs() < 1e-10 {
            return if m == 0 { 1.0 } else { 0.0 };
        }
        let x2 = x / 2.0;
        let mut sum = 0.0_f64;
        let mut x_pow = x2.powi(m);

        for k in 0..30 {
            let k_fact = Self::factorial(k as f64);
            let mk_fact = Self::factorial((m + k) as f64);
            if mk_fact.is_infinite() || k_fact.is_infinite() || x_pow.is_infinite() {
                break;
            }
            let denom = k_fact * mk_fact;
            if denom.abs() < 1e-20 {
                break;
            }
            let term = x_pow / denom;
            sum += term;
            if term.abs() < 1e-12 * sum.abs().max(1.0) {
                break;
            }
            x_pow *= x2 * x2;
        }
        sum
    }

    pub fn compute_modes_for_drum(&self, drum: &EthnicDrumProfile) -> Vec<(usize, usize, f64, f64)> {
        let ac = self.ac();
        let a = drum.diameter_cm / 200.0;
        let h = drum.avg_thickness_mm / 1000.0;
        let rho = 8700.0_f64;
        let nu = 0.34_f64;
        let e = 1.1e11_f64;
        let d = e * h.powi(3) / (12.0 * (1.0 - nu.powi(2)));

        let curvature_factor = 1.0 + 0.15 * (a / drum.shell_curvature_radius_m.max(0.1));
        let thickness_taper = if drum.thickness_center_mm > drum.thickness_edge_mm {
            1.0 + 0.08 * (drum.thickness_center_mm - drum.thickness_edge_mm) / drum.avg_thickness_mm.max(0.1)
        } else if drum.thickness_edge_mm > drum.thickness_center_mm {
            1.0 - 0.05 * (drum.thickness_edge_mm - drum.thickness_center_mm) / drum.avg_thickness_mm.max(0.1)
        } else {
            1.0
        };
        let roughness_damping = 0.001 * (drum.surface_roughness_um / 10.0).min(5.0);

        ac.eigenvalues_lambda
            .iter()
            .filter(|ev| ev.m <= 4 && ev.n <= 4)
            .map(|ev| {
                let f_raw = ev.lambda.powi(2) / (2.0 * std::f64::consts::PI * a.powi(2))
                    * (d / (rho * h)).sqrt();
                let f = f_raw * curvature_factor * thickness_taper;
                let damping = 0.005 + (ev.n as f64) * 0.002 + roughness_damping;
                (ev.m, ev.n, f, damping)
            })
            .take(15)
            .collect()
    }

    pub fn compute_spectrum_for_drum(&self, drum: &EthnicDrumProfile) -> Vec<SpectrumBin> {
        let modes = self.compute_modes_for_drum(drum);
        let weights: Vec<f64> = modes
            .iter()
            .map(|(m, _n, _f, _damp)| {
                if *m == 0 { 1.0 } else { 0.6 }
            })
            .collect();
        Self::synthesize_spectrum(&modes, &weights)
    }

    pub fn synthesize_spectrum(
        modes: &[(usize, usize, f64, f64)],
        weights: &[f64],
    ) -> Vec<SpectrumBin> {
        let mut spectrum = Vec::new();
        let max_f = 3000.0;
        let bin_size = 5.0;
        let num_bins = (max_f / bin_size) as usize;

        for i in 0..num_bins {
            let center_freq = (i as f64 + 0.5) * bin_size;
            let mut amplitude = 0.0;

            for ((_m, _n, f, damp), &weight) in modes.iter().zip(weights.iter()) {
                if weight < 0.01 {
                    continue;
                }
                let width = f * damp + 2.0;
                let diff = (center_freq - f).abs();
                if diff < width * 4.0 {
                    let gaussian = (- (diff * diff) / (2.0 * width * width)).exp();
                    amplitude += gaussian * weight;
                }
            }

            spectrum.push(SpectrumBin {
                frequency_hz: center_freq,
                amplitude_db: if amplitude > 0.0 {
                    20.0 * amplitude.log10() + 80.0
                } else {
                    -60.0
                },
            });
        }
        spectrum
    }

    pub fn estimate_sound_power(&self, drum: &EthnicDrumProfile) -> f64 {
        let modes = self.compute_modes_for_drum(drum);
        let a = drum.diameter_cm / 200.0;
        let area = std::f64::consts::PI * a * a;
        let avg_freq = modes.iter().map(|(_, _, f, _)| *f).sum::<f64>() / modes.len() as f64;
        let rho0 = self.ac().air_density_kgm3;
        let c0 = self.ac().air_sound_speed_ms;
        let rad_impedance = rho0 * c0 * area;
        let rad_efficiency = (2.0 * std::f64::consts::PI * avg_freq * a / c0).min(1.0);
        rad_impedance * 0.001 * rad_efficiency
    }

    pub async fn compute_modes_async(&self, drum: EthnicDrumProfile) -> Vec<(usize, usize, f64, f64)> {
        let pool_copy = self.config.clone();
        task::spawn_blocking(move || {
            let local_pool = AcousticFemPool::new(pool_copy);
            local_pool.compute_modes_for_drum(&drum)
        }).await.unwrap_or_default()
    }

    pub async fn compute_spectrum_async(&self, drum: EthnicDrumProfile) -> Vec<SpectrumBin> {
        let pool_copy = self.config.clone();
        task::spawn_blocking(move || {
            let local_pool = AcousticFemPool::new(pool_copy);
            local_pool.compute_spectrum_for_drum(&drum)
        }).await.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn make_pool() -> AcousticFemPool {
        let config = crate::config::AppConfig::load();
        AcousticFemPool::new(Arc::new(config))
    }

    #[test]
    fn test_factorial_basic() {
        assert_relative_eq!(AcousticFemPool::factorial(0.0), 1.0);
        assert_relative_eq!(AcousticFemPool::factorial(5.0), 120.0);
        assert_relative_eq!(AcousticFemPool::factorial(-1.0), 1.0);
    }

    #[test]
    fn test_bessel_j_at_zero() {
        assert_relative_eq!(AcousticFemPool::bessel_j(0, 0.0), 1.0);
        for m in 1..=5 {
            assert_relative_eq!(AcousticFemPool::bessel_j(m, 0.0), 0.0);
        }
    }

    #[test]
    fn test_bessel_j_known() {
        assert_relative_eq!(AcousticFemPool::bessel_j(0, 1.0),
            0.7651976865, max_relative = 1e-5);
        assert_relative_eq!(AcousticFemPool::bessel_j(1, 1.0),
            0.4400505857, max_relative = 1e-5);
    }

    #[test]
    fn test_bessel_i_at_zero() {
        assert_relative_eq!(AcousticFemPool::bessel_i(0, 0.0), 1.0);
        for m in 1..=5 {
            assert_relative_eq!(AcousticFemPool::bessel_i(m, 0.0), 0.0);
        }
    }

    #[test]
    fn test_get_lambda_positive() {
        let pool = make_pool();
        let l1 = pool.get_lambda(0, 1);
        let l2 = pool.get_lambda(2, 2);
        assert!(l1 > 0.0 && l2 > l1);
    }

    #[test]
    fn test_compute_modes_produces_positive_freqs() {
        let pool = make_pool();
        let profile = crate::models::EthnicDrumProfile {
            drum_id: "zhuang-dagu".into(), name: "壮族大铜鼓".into(),
            ethnic_group: "壮族".into(), origin_region: "广西左江流域".into(),
            estimated_era: "战国-西汉".into(), diameter_cm: 100.0,
            height_cm: 60.0, mass_kg: 35.0,
            alloy: crate::models::AlloyComposition {
                copper_pct: 82.0, tin_pct: 12.0, lead_pct: 4.5, zinc_pct: 0.8, other_impurities_pct: 0.7,
            },
            thickness_profile: "central_thick".into(), avg_thickness_mm: 4.5,
            cultural_significance: "x".into(), traditional_uses: vec![],
            shell_curvature_radius_m: 2.5, thickness_center_mm: 5.8,
            thickness_edge_mm: 3.2, casting_method: "泥范法".into(), surface_roughness_um: 15.0,
        };
        let modes = pool.compute_modes_for_drum(&profile);
        assert!(!modes.is_empty());
        for (_, _, f, d) in &modes {
            assert!(*f > 0.0);
            assert!(*d > 0.0);
        }
    }

    #[test]
    fn test_synthesize_spectrum_has_bins() {
        let modes = vec![(0, 1, 200.0, 0.01), (1, 1, 340.0, 0.015)];
        let weights = vec![1.0, 0.8];
        let bins = AcousticFemPool::synthesize_spectrum(&modes, &weights);
        assert!(!bins.is_empty());
        for b in &bins {
            assert!(b.frequency_hz >= 0.0);
        }
    }
}
