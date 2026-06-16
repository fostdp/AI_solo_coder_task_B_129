use crate::acoustic_fem_pool::AcousticFemPool;
use crate::config::AppConfig;
use crate::ethnic_comparator::EthnicComparator;
use crate::models::*;
use std::sync::Arc;

pub struct EraComparator {
    config: Arc<AppConfig>,
    pool: Arc<AcousticFemPool>,
    ethnic: Arc<EthnicComparator>,
}

impl EraComparator {
    pub fn new(config: Arc<AppConfig>, pool: Arc<AcousticFemPool>, ethnic: Arc<EthnicComparator>) -> Self {
        Self { config, pool, ethnic }
    }

    pub fn get_timpani_profile(&self, size_inches: f64) -> TimpaniDrumProfile {
        let clamped: f64 = size_inches.clamp(20.0, 32.0);
        let (name, material, tension, damping, bowl_depth, membrane_type, fundamental) =
            if clamped <= 21.0 {
                ("20-21in 高音定音鼓", "黄铜碗体+专业合成膜", 12000.0, 0.008, 23.0, "Calfskin premium membrane", 523.25 * (29.0_f64 / 21.0).sqrt())
            } else if clamped <= 24.0 {
                ("23-24in 次高音定音鼓", "铜合金碗体+PET膜", 11000.0, 0.009, 26.0, "Synthetic PET film", 523.25 * (29.0_f64 / 24.0).sqrt())
            } else if clamped <= 27.0 {
                ("26-27in 中音定音鼓", "铜合金碗体+PET膜", 9000.0, 0.011, 29.0, "Synthetic PET film", 523.25 * (29.0_f64 / 27.0).sqrt())
            } else if clamped <= 29.0 {
                ("28-29in 次低音定音鼓", "铜制碗体+聚碳酸酯膜", 7000.0, 0.013, 32.0, "Kevlar reinforced calfskin", 523.25 * (29.0_f64 / 29.0).sqrt())
            } else {
                ("30-32in 低音定音鼓", "铜制碗体+聚碳酸酯膜", 5500.0, 0.015, 35.0, "Kevlar reinforced calfskin", 523.25 * (29.0_f64 / 32.0).sqrt())
            };

        let freq_range = (fundamental * 0.85, fundamental * 1.15);
        TimpaniDrumProfile {
            name: name.to_string(),
            diameter_inches: clamped,
            diameter_cm: clamped * 2.54,
            fundamental_freq_hz: fundamental,
            freq_range_hz: freq_range,
            material: material.to_string(),
            tension_pascals: tension,
            damping_ratio: damping,
            harmonic_structure: vec![1.0, 1.505, 1.999, 2.492, 3.0, 3.506, 4.004],
            standard_reference: "ISO 285:1994 Musical instruments — Timpani — Mechanical and acoustical requirements".to_string(),
            bowl_depth_cm: bowl_depth,
            membrane_type: membrane_type.to_string(),
        }
    }

    pub fn compute_timpani_spectrum(&self, profile: &TimpaniDrumProfile) -> Vec<SpectrumBin> {
        let num_harmonics = 10;
        let mut bins = Vec::with_capacity(num_harmonics);
        for i in 1..=num_harmonics {
            let harmonic_ratio = if i <= profile.harmonic_structure.len() {
                profile.harmonic_structure[i - 1]
            } else {
                i as f64
            };
            let freq = profile.fundamental_freq_hz * harmonic_ratio;
            let base_amp = -3.0 * (i as f64).ln();
            let damping_factor = -profile.damping_ratio * (i as f64) * 3.0;
            let amp = (120.0 + base_amp + damping_factor).clamp(20.0, 130.0);
            bins.push(SpectrumBin { frequency_hz: freq, amplitude_db: amp });
        }
        bins
    }

    pub fn cross_era_comparison(
        &self,
        ancient_drum_id: &str,
        timpani_size_inches: Option<f64>,
    ) -> Option<CrossEraComparison> {
        let ancient = self.ethnic.get_ethnic_drum_profile(ancient_drum_id)?;
        let size = timpani_size_inches.unwrap_or(29.0);
        let modern = self.get_timpani_profile(size);

        let ancient_modes = self.pool.compute_modes_for_drum(&ancient);
        let ancient_fund = ancient_modes.first().map(|(_, _, f, _)| *f).unwrap_or(80.0);
        let ancient_spectrum = self.pool.compute_spectrum_for_drum(&ancient);
        let modern_spectrum = self.compute_timpani_spectrum(&modern);
        let modern_fundamental = modern.fundamental_freq_hz;
        let modern_harm_struct = modern.harmonic_structure.clone();
        let modern_damping = modern.damping_ratio;
        let modern_tension = modern.tension_pascals;

        let ancient_harmonic_ratios: Vec<f64> = ancient_modes
            .iter()
            .map(|(_, _, f, _)| f / ancient_fund.max(1.0))
            .take(8)
            .collect();

        let modern_harmonic_ratios: Vec<f64> = modern_harm_struct
            .iter()
            .take(8)
            .cloned()
            .collect();

        let avg_damping_ancient: f64 = ancient_modes.iter()
            .map(|(_, _, _, d)| *d).sum::<f64>() / ancient_modes.len().max(1) as f64;
        let ancient_decay = if avg_damping_ancient > 0.0 { 1.0 / avg_damping_ancient } else { 2.0 };
        let modern_decay = if modern_damping > 0.0 { 1.0 / modern_damping } else { 3.0 };

        let ancient_brightness = ancient_modes.iter()
            .filter(|(_, _, f, _)| *f > ancient_fund * 2.0)
            .count() as f64 / ancient_modes.len().max(1) as f64;
        let modern_brightness = 0.68;

        let ancient_radiated = self.pool.estimate_sound_power(&ancient);
        let modern_radiated = modern_tension * 0.00008;

        Some(CrossEraComparison {
            ancient_drum: ancient,
            modern_drum: modern,
            metrics_comparison: CrossEraMetrics {
                ancient_fundamental_hz: ancient_fund,
                modern_fundamental_hz: modern_fundamental,
                ancient_harmonic_ratios,
                modern_harmonic_ratios,
                ancient_decay_s: ancient_decay,
                modern_decay_s: modern_decay,
                ancient_brightness,
                modern_brightness,
                ancient_radiated_power_w: ancient_radiated,
                modern_radiated_power_w: modern_radiated,
                pitch_tunability: "古代铜鼓：铸造后不可调；现代定音鼓：踏板连续调半音~六度".to_string(),
                cultural_context: "古代铜鼓用于祭祀、仪式、权力象征；现代定音鼓用于交响乐团、室内乐".to_string(),
            },
            ancient_spectrum,
            modern_spectrum,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> EraComparator {
        let config = Arc::new(crate::config::AppConfig::load());
        let pool = Arc::new(AcousticFemPool::new(config.clone()));
        let ethnic = Arc::new(EthnicComparator::new(config.clone(), pool.clone()));
        EraComparator::new(config, pool, ethnic)
    }

    #[test]
    fn test_timpani_larger_size_lower_freq() {
        let c = make();
        let f20 = c.get_timpani_profile(20.0).fundamental_freq_hz;
        let f29 = c.get_timpani_profile(29.0).fundamental_freq_hz;
        let f32 = c.get_timpani_profile(32.0).fundamental_freq_hz;
        assert!(f20 > f29, "20in({f20}) should have higher freq than 29in({f29})");
        assert!(f29 > f32, "29in({f29}) should have higher freq than 32in({f32})");
    }

    #[test]
    fn test_timpani_modes_are_harmonic() {
        let c = make();
        let p = c.get_timpani_profile(29.0);
        for (i, r) in p.harmonic_structure.iter().enumerate() {
            let harmonic_num = (i + 1) as f64;
            assert!(*r > harmonic_num * 0.5 && *r < harmonic_num * 2.0,
                "harmonic {i}: ratio={r} should be ~{harmonic_num}x fundamental");
        }
    }

    #[test]
    fn test_cross_era_returns_some_for_valid() {
        let c = make();
        let r = c.cross_era_comparison("zhuang-dagu", Some(29.0));
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.metrics_comparison.modern_harmonic_ratios.len() > 0);
        assert!(r.metrics_comparison.ancient_fundamental_hz > 0.0);
    }

    #[test]
    fn test_cross_era_returns_none_for_invalid() {
        let c = make();
        assert!(c.cross_era_comparison("nonexistent", Some(29.0)).is_none());
    }

    #[test]
    fn test_cross_era_modern_higher_harmonicity() {
        let c = make();
        let r = c.cross_era_comparison("zhuang-dagu", Some(29.0)).unwrap();
        let anom = r.metrics_comparison.ancient_harmonic_ratios.iter()
            .enumerate()
            .map(|(i, r)| (r - (i + 1) as f64).abs())
            .sum::<f64>();
        let mnom = r.metrics_comparison.modern_harmonic_ratios.iter()
            .enumerate()
            .map(|(i, r)| (r - (i + 1) as f64).abs())
            .sum::<f64>();
        assert!(mnom < anom, "modern harmonic deviation ({mnom}) < ancient ({anom})");
    }
}
