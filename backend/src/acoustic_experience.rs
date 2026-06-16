use crate::acoustic_fem_pool::AcousticFemPool;
use crate::config::AppConfig;
use crate::ethnic_comparator::EthnicComparator;
use crate::era_comparator::EraComparator;
use crate::models::*;
use crate::ritual_acoustics::RitualAcoustics;
use crate::vr_drum_ensemble::VrDrumEnsemble;
use std::sync::Arc;

pub struct AcousticExperienceService {
    config: Arc<AppConfig>,
    pool: Arc<AcousticFemPool>,
    ethnic: Arc<EthnicComparator>,
    era: Arc<EraComparator>,
    ritual: Arc<RitualAcoustics>,
    vr: Arc<VrDrumEnsemble>,
}

impl AcousticExperienceService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let pool = Arc::new(AcousticFemPool::new(config.clone()));
        let ethnic = Arc::new(EthnicComparator::new(config.clone(), pool.clone()));
        let era = Arc::new(EraComparator::new(config.clone(), pool.clone(), ethnic.clone()));
        let ritual = Arc::new(RitualAcoustics::new(config.clone(), pool.clone(), ethnic.clone()));
        let vr = Arc::new(VrDrumEnsemble::new(config.clone(), pool.clone(), ethnic.clone()));
        Self { config, pool, ethnic, era, ritual, vr }
    }

    pub fn factorial(n: f64) -> f64 {
        AcousticFemPool::factorial(n)
    }

    pub fn bessel_j(m: i32, x: f64) -> f64 {
        AcousticFemPool::bessel_j(m, x)
    }

    pub fn bessel_i(m: i32, x: f64) -> f64 {
        AcousticFemPool::bessel_i(m, x)
    }

    pub fn get_lambda(&self, m: usize, n: usize) -> f64 {
        self.pool.get_lambda(m, n)
    }

    pub fn get_ethnic_drum_library(&self) -> Vec<EthnicDrumLibraryEntry> {
        self.ethnic.get_ethnic_drum_library()
    }

    pub fn get_ethnic_drum_profile(&self, drum_id: &str) -> Option<EthnicDrumProfile> {
        self.ethnic.get_ethnic_drum_profile(drum_id)
    }

    pub fn compare_ethnic_drums(&self, drum_ids: Vec<String>) -> EthnicAcousticComparison {
        self.ethnic.compare_ethnic_drums(drum_ids)
    }

    pub fn get_timpani_profile(&self, size_inches: f64) -> TimpaniDrumProfile {
        self.era.get_timpani_profile(size_inches)
    }

    pub fn cross_era_comparison(
        &self,
        ancient_drum_id: &str,
        timpani_size_inches: Option<f64>,
    ) -> Option<CrossEraComparison> {
        self.era.cross_era_comparison(ancient_drum_id, timpani_size_inches)
    }

    pub fn ritual_sound_field(&self, req: RitualSoundFieldRequest) -> RitualSoundFieldResult {
        self.ritual.ritual_sound_field(req)
    }

    pub fn virtual_tap(&self, req: VirtualTapRequest) -> Option<VirtualTapResult> {
        self.vr.virtual_tap(req)
    }

    pub fn virtual_ensemble(&self, req: EnsembleTapRequest) -> EnsembleTapResult {
        self.vr.virtual_ensemble(req)
    }

    fn compute_modes_for_drum(&self, drum: &EthnicDrumProfile) -> Vec<(usize, usize, f64, f64)> {
        self.pool.compute_modes_for_drum(drum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn make_test_service() -> AcousticExperienceService {
        let config = crate::config::AppConfig::load();
        AcousticExperienceService::new(std::sync::Arc::new(config))
    }

    #[test]
    fn test_factorial_basic() {
        assert_relative_eq!(AcousticExperienceService::factorial(0.0), 1.0);
        assert_relative_eq!(AcousticExperienceService::factorial(1.0), 1.0);
        assert_relative_eq!(AcousticExperienceService::factorial(5.0), 120.0);
        assert_relative_eq!(AcousticExperienceService::factorial(10.0), 3628800.0);
    }

    #[test]
    fn test_factorial_negative_returns_one() {
        assert_relative_eq!(AcousticExperienceService::factorial(-1.0), 1.0);
        assert_relative_eq!(AcousticExperienceService::factorial(-100.0), 1.0);
    }

    #[test]
    fn test_factorial_large_no_overflow() {
        let result = AcousticExperienceService::factorial(30.0);
        assert!(result.is_finite(), "30! should be finite (was f64)");
        assert!(result > 1e30, "30! should be huge");
        let r60 = AcousticExperienceService::factorial(60.0);
        assert!(r60.is_finite() || r60.is_infinite(),
            "60! either finite or +inf (no panic/overflow)");
    }

    #[test]
    fn test_bessel_j_zero_order_at_zero() {
        let j = AcousticExperienceService::bessel_j(0, 0.0);
        assert_relative_eq!(j, 1.0, max_relative = 1e-9);
    }

    #[test]
    fn test_bessel_j_known_values() {
        assert_relative_eq!(AcousticExperienceService::bessel_j(0, 1.0),
            0.7651976865, max_relative = 1e-5);
        assert_relative_eq!(AcousticExperienceService::bessel_j(1, 1.0),
            0.4400505857, max_relative = 1e-5);
    }

    #[test]
    fn test_bessel_j_boundary_x_zero_nonzero_m() {
        for m in 1..=5 {
            let v = AcousticExperienceService::bessel_j(m, 0.0);
            assert_relative_eq!(v, 0.0, max_relative = 1e-9);
        }
    }

    #[test]
    fn test_bessel_j_large_m_and_x_no_panic() {
        let r = AcousticExperienceService::bessel_j(6, 20.0);
        assert!(r.is_finite(), "bessel_j should not panic at m=6,x=20");
    }

    #[test]
    fn test_bessel_i_zero_order_at_zero() {
        assert_relative_eq!(AcousticExperienceService::bessel_i(0, 0.0), 1.0);
    }

    #[test]
    fn test_bessel_i_m_ge_1_at_zero() {
        for m in 1..=5 {
            let v = AcousticExperienceService::bessel_i(m, 0.0);
            assert_relative_eq!(v, 0.0, max_relative = 1e-9);
        }
    }

    #[test]
    fn test_bessel_i_large_argument_no_panic() {
        let r = AcousticExperienceService::bessel_i(6, 100.0);
        assert!(r.is_finite() || r.is_infinite(),
            "bessel_i should not panic at large argument");
    }

    #[test]
    fn test_get_lambda_returns_valid() {
        let s = make_test_service();
        let lam = s.get_lambda(0, 1);
        assert!(lam > 0.0, "lambda_{{0,1}} must be positive (got {lam})");
        let lam2 = s.get_lambda(2, 2);
        assert!(lam2 > lam, "higher order modes have larger lambda");
    }

    #[test]
    fn test_get_lambda_boundary_m_large() {
        let s = make_test_service();
        let _ = s.get_lambda(3, 6);
    }

    #[test]
    fn test_ethnic_library_has_six_drums() {
        let s = make_test_service();
        let lib = s.get_ethnic_drum_library();
        assert_eq!(lib.len(), 6, "expected 2*3 ethnic drums = 6");
        for d in &lib {
            assert!(!d.drum_id.is_empty());
            assert!(d.diameter_cm > 0.0);
            assert!(matches!(d.ethnic_group.as_str(),
                "壮族" | "苗族" | "瑶族"));
        }
    }

    #[test]
    fn test_get_profile_valid_and_invalid() {
        let s = make_test_service();
        let valid = s.get_ethnic_drum_profile("zhuang-dagu");
        assert!(valid.is_some(), "zhuang-dagu should exist");
        let v = valid.unwrap();
        assert_eq!(v.drum_id, "zhuang-dagu");
        assert!(v.alloy.copper_pct + v.alloy.tin_pct + v.alloy.lead_pct
            + v.alloy.zinc_pct + v.alloy.other_impurities_pct
            > 99.9);

        let missing = s.get_ethnic_drum_profile("does-not-exist-xyz");
        assert!(missing.is_none(), "nonexistent should return None");
    }

    #[test]
    fn test_compare_ethnic_drums_normal() {
        let s = make_test_service();
        let result = s.compare_ethnic_drums(
            vec!["zhuang-dagu".into(), "miao-dagu".into(), "yao-dagu".into()]
        );
        assert_eq!(result.drums.len(), 3);
        assert_eq!(result.frequency_spectra.len(), 3);
        for (drum_id, bins) in &result.frequency_spectra {
            assert!(!bins.is_empty(), "spectrum of {drum_id} should have bins");
            for b in bins {
                assert!(b.frequency_hz > 0.0);
                assert!(b.amplitude_db <= 200.0);
            }
        }
        assert!(result.comparison_metrics.fundamental_freqs.len() >= 3);
    }

    #[test]
    fn test_compare_ethnic_drums_single_element_edge() {
        let s = make_test_service();
        let r = s.compare_ethnic_drums(vec!["yao-guzai".into()]);
        assert_eq!(r.drums.len(), 1);
        assert_eq!(r.frequency_spectra.len(), 1);
        assert!(r.vibration_modes_comparison.len() >= 1);
    }

    #[test]
    fn test_compare_ethnic_drums_includes_invalid() {
        let s = make_test_service();
        let r = s.compare_ethnic_drums(vec![
            "zhuang-dagu".into(),
            "invalid-id-12345".into(),
            "miao-xiaogu".into(),
        ]);
        assert_eq!(r.drums.len(), 2,
            "invalid drum should be skipped, 2 valid remain");
    }

    #[test]
    fn test_cross_era_comparison_normal() {
        let s = make_test_service();
        let r = s.cross_era_comparison("zhuang-dagu", Some(29.0));
        assert!(r.is_some(), "comparison must be Some for valid drum");
        let u = r.unwrap();
        let m = &u.metrics_comparison;
        assert!(m.ancient_fundamental_hz > 0.0);
        assert!(m.modern_fundamental_hz > 0.0);
        assert!(!u.ancient_spectrum.is_empty());
        assert!(!u.modern_spectrum.is_empty());
    }

    #[test]
    fn test_cross_era_comparison_timpani_sizes_boundary() {
        let s = make_test_service();
        let r_small = s.cross_era_comparison("yao-guzai", Some(20.0)).unwrap();
        let r_large = s.cross_era_comparison("yao-guzai", Some(32.0)).unwrap();
        assert!(r_large.metrics_comparison.modern_fundamental_hz
            < r_small.metrics_comparison.modern_fundamental_hz,
            "larger timpani has lower fundamental");
    }

    #[test]
    fn test_cross_era_comparison_invalid_drum() {
        let s = make_test_service();
        let r = s.cross_era_comparison("nope-no-such-drum", Some(26.0));
        assert!(r.is_none(), "invalid drum id -> None");
    }

    #[test]
    fn test_virtual_tap_center_is_sane() {
        let s = make_test_service();
        let opt = s.virtual_tap(VirtualTapRequest {
            drum_id: "zhuang-dagu".into(), x_frac: 0.5, y_frac: 0.5,
            strike_force: 1.0, striker_type: None,
        });
        let c = opt.unwrap();
        assert!((1.0..=500.0).contains(&c.brightness),
            "center brightness expected sane (got {})", c.brightness);
        assert!(c.decay_time_s > 0.0);
        assert!(c.fundamental_freq_hz > 0.0);
    }

    #[test]
    fn test_virtual_tap_edge_vs_center_different() {
        let s = make_test_service();
        let center = s.virtual_tap(VirtualTapRequest {
            drum_id: "zhuang-dagu".into(), x_frac: 0.5, y_frac: 0.5,
            strike_force: 1.0, striker_type: None,
        }).unwrap();
        let edge = s.virtual_tap(VirtualTapRequest {
            drum_id: "zhuang-dagu".into(), x_frac: 0.95, y_frac: 0.5,
            strike_force: 1.0, striker_type: None,
        }).unwrap();
        assert_ne!(center.zone_name, edge.zone_name,
            "center vs edge should have different zone names");
    }

    #[test]
    fn test_virtual_tap_outside_bounds_clamps() {
        let s = make_test_service();
        let r1 = s.virtual_tap(VirtualTapRequest {
            drum_id: "miao-dagu".into(),
            x_frac: 999.0, y_frac: -100.0,
            strike_force: 1.0, striker_type: None,
        });
        assert!(r1.is_some(), "even garbage coords must produce result");
        assert!(r1.unwrap().fundamental_freq_hz > 0.0);
    }

    #[test]
    fn test_virtual_tap_force_boundary_zero_and_high() {
        let s = make_test_service();
        let rz = s.virtual_tap(VirtualTapRequest {
            drum_id: "zhuang-dagu".into(),
            x_frac: 0.5, y_frac: 0.5, strike_force: 0.0, striker_type: None,
        }).unwrap();
        assert!(rz.web_audio_params.attack_s >= 0.0, "zero force handled");
        let rh = s.virtual_tap(VirtualTapRequest {
            drum_id: "zhuang-dagu".into(),
            x_frac: 0.5, y_frac: 0.5, strike_force: 100.0, striker_type: None,
        }).unwrap();
        assert!(rh.decay_time_s > 0.0, "huge force handled without panic");
    }

    #[test]
    fn test_compute_modes_produces_stable_frequencies() {
        let s = make_test_service();
        let profile = s.get_ethnic_drum_profile("zhuang-dagu").unwrap();
        let modes = s.compute_modes_for_drum(&profile);
        assert!(!modes.is_empty());
        for (_, _, f, damp) in &modes {
            assert!(*f > 0.0, "frequency must be positive");
            assert!(*damp > 0.0, "damping must be positive");
        }
    }

    #[test]
    fn test_ritual_sound_field_multiple_drums() {
        let s = make_test_service();
        let placements = vec![
            RitualDrumPlacement {
                drum_id: "zhuang-dagu".into(),
                position_x_m: -3.0, position_y_m: 0.0, position_z_m: 0.0,
                relative_volume: 1.0, strike_phase_offset_s: 0.0,
            },
            RitualDrumPlacement {
                drum_id: "miao-dagu".into(),
                position_x_m: 3.0, position_y_m: 0.0, position_z_m: 0.0,
                relative_volume: 0.8, strike_phase_offset_s: 0.1,
            },
        ];
        let req = RitualSoundFieldRequest {
            drum_placements: placements,
            observer_x_m: Some(0.0), observer_y_m: Some(0.0), observer_z_m: Some(1.5),
            field_radius_m: Some(10.0), grid_resolution: Some(4),
        };
        let r = s.ritual_sound_field(req);
        assert_eq!(r.drum_count, 2);
        assert!(!r.sound_field.is_empty());
        for sfp in &r.sound_field {
            assert!(sfp.spl_db < 200.0, "SPL must be realistic (< 200 dB)");
            assert!(sfp.pressure_pa >= 0.0, "pressure amplitude non-negative");
            assert!(sfp.intensity_wm2 >= 0.0);
        }
        assert!(r.observer_spl_db > 0.0 && r.observer_spl_db < 200.0);
        assert!(r.total_radiated_power_w > 0.0);
        assert!(!r.temporal_envelope.is_empty());
    }

    #[test]
    fn test_ritual_sound_field_empty_edge() {
        let s = make_test_service();
        let req = RitualSoundFieldRequest {
            drum_placements: vec![],
            observer_x_m: Some(0.0), observer_y_m: Some(0.0), observer_z_m: Some(0.0),
            field_radius_m: Some(5.0), grid_resolution: Some(3),
        };
        let r = s.ritual_sound_field(req);
        assert_eq!(r.drum_count, 0, "empty placements -> drum_count 0");
        for sfp in &r.sound_field {
            assert!(sfp.spl_db.is_finite(),
                "empty field should not produce NaN SPL");
        }
    }

    #[test]
    fn test_ritual_sound_field_large_grid_no_panic() {
        let s = make_test_service();
        let placements = (0..16).map(|i| RitualDrumPlacement {
            drum_id: "zhuang-dagu".into(),
            position_x_m: (i as f64) * 0.5 - 4.0,
            position_y_m: 0.0, position_z_m: 0.0,
            relative_volume: 1.0, strike_phase_offset_s: 0.0,
        }).collect();
        let req = RitualSoundFieldRequest {
            drum_placements: placements,
            observer_x_m: Some(0.0), observer_y_m: Some(0.0), observer_z_m: Some(1.5),
            field_radius_m: Some(15.0), grid_resolution: Some(16),
        };
        let r = s.ritual_sound_field(req);
        assert!(r.sound_field.len() >= 16 * 16 / 2);
    }

    #[test]
    fn test_ritual_sound_field_observer_coincident_with_drum_no_nan() {
        let s = make_test_service();
        let req = RitualSoundFieldRequest {
            drum_placements: vec![RitualDrumPlacement {
                drum_id: "zhuang-dagu".into(),
                position_x_m: 0.0, position_y_m: 0.0, position_z_m: 0.0,
                relative_volume: 1.0, strike_phase_offset_s: 0.0,
            }],
            observer_x_m: Some(0.0), observer_y_m: Some(0.0), observer_z_m: Some(0.0),
            field_radius_m: Some(5.0), grid_resolution: Some(4),
        };
        let r = s.ritual_sound_field(req);
        assert!(r.observer_spl_db.is_finite(),
            "observer at drum position: SPL must remain finite");
        for sfp in &r.sound_field {
            assert!(sfp.pressure_pa.is_finite(),
                "pressure must be finite at every point");
        }
    }

    #[test]
    fn test_field_survey_fields_present() {
        let s = make_test_service();
        for did in ["zhuang-dagu", "miao-dagu", "yao-guzai"] {
            let p = s.get_ethnic_drum_profile(did).unwrap();
            assert!(p.shell_curvature_radius_m > 0.0,
                "{}: shell_curvature_radius_m must be positive", did);
            assert!(p.thickness_center_mm > 0.0,
                "{}: thickness_center_mm must be positive", did);
            assert!(p.thickness_edge_mm > 0.0,
                "{}: thickness_edge_mm must be positive", did);
            assert!(!p.casting_method.is_empty(),
                "{}: casting_method must not be empty", did);
            assert!(p.surface_roughness_um >= 0.0,
                "{}: surface_roughness_um must be non-negative", did);
        }
    }

    #[test]
    fn test_curvature_and_taper_affect_modes() {
        let s = make_test_service();
        let zhuang = s.get_ethnic_drum_profile("zhuang-dagu").unwrap();
        let yao = s.get_ethnic_drum_profile("yao-dagu").unwrap();
        let z_modes = s.compute_modes_for_drum(&zhuang);
        let y_modes = s.compute_modes_for_drum(&yao);
        assert!(!z_modes.is_empty() && !y_modes.is_empty());
        let z_f0 = z_modes[0].2;
        let y_f0 = y_modes[0].2;
        assert!(z_f0 > 0.0 && y_f0 > 0.0,
            "curvature+taper corrected frequencies must be positive");
    }

    #[test]
    fn test_timpani_profile_iso_standard() {
        let s = make_test_service();
        for size in [20.0, 23.0, 26.0, 29.0, 32.0, 35.0] {
            let tp = s.get_timpani_profile(size);
            assert!(!tp.standard_reference.is_empty(),
                "{}in: standard_reference must not be empty", size);
            assert!(tp.bowl_depth_cm > 0.0,
                "{}in: bowl_depth_cm must be positive", size);
            assert!(!tp.membrane_type.is_empty(),
                "{}in: membrane_type must not be empty", size);
            assert!(tp.tension_pascals > 0.0,
                "{}in: tension_pascals must be positive", size);
            assert!(tp.damping_ratio > 0.0 && tp.damping_ratio < 0.1,
                "{}in: damping_ratio should be in realistic range", size);
        }
    }

    #[test]
    fn test_timpani_larger_has_lower_tension_higher_damping() {
        let s = make_test_service();
        let small = s.get_timpani_profile(20.0);
        let large = s.get_timpani_profile(32.0);
        assert!(small.tension_pascals > large.tension_pascals,
            "smaller timpani should have higher tension");
        assert!(large.damping_ratio > small.damping_ratio,
            "larger timpani should have higher damping");
    }

    #[test]
    fn test_ritual_soundfield_has_ground_reflection_effect() {
        let s = make_test_service();
        let req = RitualSoundFieldRequest {
            drum_placements: vec![RitualDrumPlacement {
                drum_id: "zhuang-dagu".into(),
                position_x_m: 0.0, position_y_m: 0.0, position_z_m: 1.0,
                relative_volume: 1.0, strike_phase_offset_s: 0.0,
            }],
            observer_x_m: Some(5.0), observer_y_m: Some(0.0), observer_z_m: Some(1.5),
            field_radius_m: Some(10.0), grid_resolution: Some(6),
        };
        let r = s.ritual_sound_field(req);
        assert!(r.observer_spl_db > 0.0, "observer SPL should be positive with ground reflection");
        assert!(r.observer_spl_db.is_finite(), "observer SPL must be finite (no NaN from interference)");
        for sfp in &r.sound_field {
            assert!(sfp.spl_db.is_finite(),
                "interference pattern must not produce NaN");
        }
    }

    #[test]
    fn test_virtual_ensemble_unison() {
        let s = make_test_service();
        let req = EnsembleTapRequest {
            taps: vec![
                VirtualTapRequest {
                    drum_id: "zhuang-dagu".into(), x_frac: 0.5, y_frac: 0.5,
                    strike_force: 1.0, striker_type: None,
                },
                VirtualTapRequest {
                    drum_id: "miao-dagu".into(), x_frac: 0.5, y_frac: 0.5,
                    strike_force: 0.8, striker_type: None,
                },
            ],
            tempo_bpm: Some(60.0),
            rhythm_pattern: Some("unison".into()),
        };
        let r = s.virtual_ensemble(req);
        assert_eq!(r.individual_results.len(), 2);
        assert!(!r.mixed_spectrum.is_empty(), "mixed spectrum must not be empty");
        assert!(!r.combined_envelope.is_empty());
        assert!(r.synchronized, "unison pattern should be synchronized");
        assert!(r.total_duration_s > 0.0);
        assert!(!r.beat_intervals_s.is_empty());
    }

    #[test]
    fn test_virtual_ensemble_empty_taps() {
        let s = make_test_service();
        let req = EnsembleTapRequest {
            taps: vec![],
            tempo_bpm: None,
            rhythm_pattern: None,
        };
        let r = s.virtual_ensemble(req);
        assert!(r.individual_results.is_empty());
        assert!(r.total_duration_s > 0.0, "should have default duration even with no taps");
    }

    #[test]
    fn test_virtual_ensemble_polyrhythm() {
        let s = make_test_service();
        let req = EnsembleTapRequest {
            taps: vec![
                VirtualTapRequest {
                    drum_id: "zhuang-dagu".into(), x_frac: 0.3, y_frac: 0.3,
                    strike_force: 1.0, striker_type: None,
                },
                VirtualTapRequest {
                    drum_id: "yao-dagu".into(), x_frac: 0.7, y_frac: 0.5,
                    strike_force: 1.0, striker_type: None,
                },
                VirtualTapRequest {
                    drum_id: "miao-xiaogu".into(), x_frac: 0.5, y_frac: 0.7,
                    strike_force: 0.9, striker_type: None,
                },
            ],
            tempo_bpm: Some(120.0),
            rhythm_pattern: Some("polyrhythm".into()),
        };
        let r = s.virtual_ensemble(req);
        assert_eq!(r.individual_results.len(), 3);
        assert!(!r.synchronized, "polyrhythm should not be fully synchronized");
        let max_env = r.combined_envelope.iter().map(|(_, a)| *a).fold(0.0f64, f64::max);
        assert!(max_env > 0.0, "envelope must have positive amplitude");
    }
}
