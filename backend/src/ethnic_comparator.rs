use crate::acoustic_fem_pool::AcousticFemPool;
use crate::config::AppConfig;
use crate::models::*;
use std::sync::Arc;

pub struct EthnicComparator {
    config: Arc<AppConfig>,
    pool: Arc<AcousticFemPool>,
}

impl EthnicComparator {
    pub fn new(config: Arc<AppConfig>, pool: Arc<AcousticFemPool>) -> Self {
        Self { config, pool }
    }

    pub fn get_ethnic_drum_library(&self) -> Vec<EthnicDrumLibraryEntry> {
        vec![
            EthnicDrumLibraryEntry {
                drum_id: "zhuang-dagu".into(),
                name: "壮族大铜鼓（麻江型）".into(),
                ethnic_group: "壮族".into(),
                diameter_cm: 100.0,
                fundamental_hz: 78.0,
                cultural_tag: "雷王祭·蛙纹·祈雨".into(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "zhuang-magu".into(),
                name: "壮族麻江铜鼓".into(),
                ethnic_group: "壮族".into(),
                diameter_cm: 60.0,
                fundamental_hz: 145.0,
                cultural_tag: "婚丧嫁娶·铜鼓舞".into(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "miao-dagu".into(),
                name: "苗族祭祀大铜鼓".into(),
                ethnic_group: "苗族".into(),
                diameter_cm: 85.0,
                fundamental_hz: 95.0,
                cultural_tag: "牯藏节·祭祖".into(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "miao-xiaogu".into(),
                name: "苗族跳月小铜鼓".into(),
                ethnic_group: "苗族".into(),
                diameter_cm: 40.0,
                fundamental_hz: 195.0,
                cultural_tag: "跳月·芦笙合奏".into(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "yao-dagu".into(),
                name: "瑶族盘王大铜鼓".into(),
                ethnic_group: "瑶族".into(),
                diameter_cm: 90.0,
                fundamental_hz: 88.0,
                cultural_tag: "盘王节·长鼓伴奏".into(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "yao-guzai".into(),
                name: "瑶族铜鼓仔".into(),
                ethnic_group: "瑶族".into(),
                diameter_cm: 30.0,
                fundamental_hz: 220.0,
                cultural_tag: "歌堂·对歌·儿童游戏".into(),
            },
        ]
    }

    pub fn get_ethnic_drum_profile(&self, drum_id: &str) -> Option<EthnicDrumProfile> {
        let library = self.get_ethnic_drum_library();
        let entry = library.iter().find(|e| e.drum_id == drum_id)?;

        let (alloy, thickness_profile, avg_thickness, cultural, uses,
             shell_curvature, thick_center, thick_edge, casting, roughness) = match drum_id {
            "zhuang-dagu" => (
                AlloyComposition {
                    copper_pct: 82.0, tin_pct: 12.0, lead_pct: 4.5, zinc_pct: 0.8, other_impurities_pct: 0.7,
                },
                "central_thick".to_string(), 4.5,
                "壮族先民视铜鼓为雷王化身，蛙纹象征雨水与丰收。".to_string(),
                vec!["祭祀雷王".to_string(), "祈雨仪式".to_string(), "丰收庆典".to_string(), "丧葬礼仪".to_string()],
                2.5, 5.8, 3.2, "泥范法失蜡铸造".to_string(), 15.0,
            ),
            "zhuang-magu" => (
                AlloyComposition {
                    copper_pct: 80.0, tin_pct: 14.0, lead_pct: 4.0, zinc_pct: 1.0, other_impurities_pct: 1.0,
                },
                "uniform".to_string(), 3.2,
                "麻江型铜鼓是广西壮族地区最常见的类型，纹饰精美。".to_string(),
                vec!["民间祭祀".to_string(), "婚丧嫁娶".to_string(), "节日庆典".to_string(), "铜鼓舞".to_string()],
                1.8, 3.5, 3.0, "泥范法合范铸造".to_string(), 20.0,
            ),
            "miao-dagu" => (
                AlloyComposition {
                    copper_pct: 84.0, tin_pct: 10.0, lead_pct: 4.0, zinc_pct: 0.5, other_impurities_pct: 1.5,
                },
                "edge_thick".to_string(), 5.0,
                "苗族铜鼓是牯藏节的核心礼器，每十三年举办一次。".to_string(),
                vec!["牯藏大典".to_string(), "祭祖仪式".to_string(), "丧葬送魂".to_string(), "芦笙伴奏".to_string()],
                2.2, 4.0, 6.5, "泥范法分段铸造".to_string(), 25.0,
            ),
            "miao-xiaogu" => (
                AlloyComposition {
                    copper_pct: 85.0, tin_pct: 11.0, lead_pct: 2.5, zinc_pct: 0.3, other_impurities_pct: 1.2,
                },
                "uniform".to_string(), 2.5,
                "小型铜鼓便于携带，是苗族跳月活动的重要乐器。".to_string(),
                vec!["跳月舞".to_string(), "青年社交".to_string(), "节日娱乐".to_string(), "芦笙合奏".to_string()],
                1.2, 2.8, 2.5, "整体失蜡铸造".to_string(), 12.0,
            ),
            "yao-dagu" => (
                AlloyComposition {
                    copper_pct: 78.0, tin_pct: 15.0, lead_pct: 5.0, zinc_pct: 0.5, other_impurities_pct: 1.5,
                },
                "central_thick".to_string(), 4.8,
                "瑶族铜鼓与长鼓并称，盘王节中铜鼓指挥长鼓节奏。".to_string(),
                vec!["盘王节祭祀".to_string(), "长鼓舞伴奏".to_string(), "耍歌堂".to_string(), "驱邪仪式".to_string()],
                2.0, 6.0, 3.5, "泥范法失蜡铸造".to_string(), 18.0,
            ),
            "yao-guzai" => (
                AlloyComposition {
                    copper_pct: 83.0, tin_pct: 12.0, lead_pct: 3.5, zinc_pct: 0.5, other_impurities_pct: 1.0,
                },
                "uniform".to_string(), 2.0,
                "铜鼓仔音色明亮清脆，是瑶族儿童游戏和青年对歌的伴奏乐器。".to_string(),
                vec!["喜庆节日".to_string(), "歌堂伴奏".to_string(), "儿童游戏".to_string(), "信号传递".to_string()],
                0.9, 2.2, 2.0, "整体失蜡铸造".to_string(), 10.0,
            ),
            _ => return None,
        };

        let (origin_region, estimated_era, height_cm, mass_kg) = match drum_id {
            "zhuang-dagu" => ("广西左江流域", "战国-西汉 (约公元前5世纪-公元1世纪)", 60.0, 35.0),
            "zhuang-magu" => ("广西麻江县及周边", "宋-明 (约公元10-17世纪)", 45.0, 20.0),
            "miao-dagu" => ("贵州黔东南雷山台江", "唐-清 (约公元7-19世纪)", 50.0, 28.0),
            "miao-xiaogu" => ("广西融水苗族自治县", "明-近代 (约公元14-20世纪)", 30.0, 8.0),
            "yao-dagu" => ("广西金秀瑶族自治县", "汉-近代 (约公元1-20世纪)", 55.0, 32.0),
            "yao-guzai" => ("广东连南瑶族自治县", "近代 (约公元19-20世纪)", 25.0, 5.0),
            _ => ("", "", 0.0, 0.0),
        };

        Some(EthnicDrumProfile {
            drum_id: entry.drum_id.clone(),
            name: entry.name.clone(),
            ethnic_group: entry.ethnic_group.clone(),
            origin_region: origin_region.to_string(),
            estimated_era: estimated_era.to_string(),
            diameter_cm: entry.diameter_cm,
            height_cm,
            mass_kg,
            alloy,
            thickness_profile,
            avg_thickness_mm: avg_thickness,
            cultural_significance: cultural,
            traditional_uses: uses,
            shell_curvature_radius_m: shell_curvature,
            thickness_center_mm: thick_center,
            thickness_edge_mm: thick_edge,
            casting_method: casting,
            surface_roughness_um: roughness,
        })
    }

    pub fn compare_ethnic_drums(&self, drum_ids: Vec<String>) -> EthnicAcousticComparison {
        let drums: Vec<EthnicDrumProfile> = drum_ids
            .iter()
            .filter_map(|id| self.get_ethnic_drum_profile(id))
            .collect();

        let frequency_spectra: Vec<(String, Vec<SpectrumBin>)> = drums
            .iter()
            .map(|d| (d.drum_id.clone(), self.pool.compute_spectrum_for_drum(d)))
            .collect();

        let vibration_modes_comparison: Vec<(String, Vec<VibrationMode>)> = drums
            .iter()
            .map(|d| {
                let modes = self.pool.compute_modes_for_drum(d);
                let modes_converted = modes
                    .iter()
                    .enumerate()
                    .map(|(i, (m, n, f, damp))| VibrationMode {
                        mode_order: i,
                        frequency_hz: *f,
                        nonlinear_frequency_hz: *f,
                        damping_ratio: *damp,
                        node_pattern: format!("({m},{n})"),
                        modal_displacements: vec![],
                        effective_force_amplitude: 1.0,
                        arc_length_converged: true,
                    })
                    .collect();
                (d.drum_id.clone(), modes_converted)
            })
            .collect();

        let comparison_metrics = self.compute_comparison_metrics(&drums);
        EthnicAcousticComparison {
            drums,
            frequency_spectra,
            vibration_modes_comparison,
            comparison_metrics,
        }
    }

    fn compute_comparison_metrics(&self, drums: &[EthnicDrumProfile]) -> EthnicComparisonMetrics {
        let fundamental_freqs: Vec<(String, f64)> = drums
            .iter()
            .map(|d| {
                let modes = self.pool.compute_modes_for_drum(d);
                (d.drum_id.clone(), modes.first().map(|(_, _, f, _)| *f).unwrap_or(0.0))
            })
            .collect();

        let radiated_powers: Vec<(String, f64)> = drums
            .iter()
            .map(|d| (d.drum_id.clone(), self.pool.estimate_sound_power(d)))
            .collect();

        let decay_times: Vec<(String, f64)> = drums
            .iter()
            .map(|d| {
                let modes = self.pool.compute_modes_for_drum(d);
                let avg_damping = modes.iter()
                    .map(|(_, _, _, d)| *d).sum::<f64>() / modes.len().max(1) as f64;
                let decay = if avg_damping > 0.0 { 1.0 / avg_damping } else { 2.0 };
                (d.drum_id.clone(), decay)
            })
            .collect();

        let harmonic_series: Vec<(String, Vec<f64>)> = drums
            .iter()
            .map(|d| {
                let modes = self.pool.compute_modes_for_drum(d);
                let fund = modes.first().map(|(_, _, f, _)| *f).unwrap_or(1.0);
                let ratios = modes.iter()
                    .map(|(_, _, f, _)| f / fund.max(1.0))
                    .collect();
                (d.drum_id.clone(), ratios)
            })
            .collect();

        let brightness: Vec<(String, f64)> = drums
            .iter()
            .map(|d| {
                let modes = self.pool.compute_modes_for_drum(d);
                let fund = modes.first().map(|(_, _, f, _)| *f).unwrap_or(1.0);
                let b = modes.iter()
                    .filter(|(_, _, f, _)| *f > fund * 2.0)
                    .count() as f64 / modes.len().max(1) as f64;
                (d.drum_id.clone(), b)
            })
            .collect();

        let sound_quality_scores: Vec<(String, f64)> = drums
            .iter()
            .map(|d| (d.drum_id.clone(), 75.0 + d.diameter_cm / 10.0))
            .collect();

        EthnicComparisonMetrics {
            fundamental_freqs,
            sound_quality_scores,
            radiated_powers,
            harmonic_series,
            decay_times_s: decay_times,
            brightness,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> EthnicComparator {
        let config = Arc::new(crate::config::AppConfig::load());
        let pool = Arc::new(AcousticFemPool::new(config.clone()));
        EthnicComparator::new(config, pool)
    }

    #[test]
    fn test_library_has_six_drums() {
        let c = make();
        let lib = c.get_ethnic_drum_library();
        assert_eq!(lib.len(), 6);
        for d in &lib {
            assert!(!d.drum_id.is_empty());
            assert!(d.fundamental_hz > 0.0);
            assert!(!d.cultural_tag.is_empty());
        }
    }

    #[test]
    fn test_profile_valid_and_invalid() {
        let c = make();
        assert!(c.get_ethnic_drum_profile("zhuang-dagu").is_some());
        assert!(c.get_ethnic_drum_profile("none").is_none());
        let p = c.get_ethnic_drum_profile("zhuang-dagu").unwrap();
        assert!(p.shell_curvature_radius_m > 0.0);
    }

    #[test]
    fn test_compare_normal() {
        let c = make();
        let r = c.compare_ethnic_drums(
            vec!["zhuang-dagu".into(), "miao-dagu".into(), "yao-dagu".into()]);
        assert_eq!(r.drums.len(), 3);
        assert_eq!(r.frequency_spectra.len(), 3);
        assert!(r.comparison_metrics.fundamental_freqs.len() >= 3);
        assert!(r.comparison_metrics.radiated_powers.len() >= 3);
    }

    #[test]
    fn test_compare_invalid_ids_filtered() {
        let c = make();
        let r = c.compare_ethnic_drums(vec!["zhuang-dagu".into(), "xxxxx".into()]);
        assert_eq!(r.drums.len(), 1);
    }

    #[test]
    fn test_profile_alloy_sum_near_100() {
        let c = make();
        for did in ["zhuang-dagu", "miao-xiaogu", "yao-guzai"] {
            let p = c.get_ethnic_drum_profile(did).unwrap();
            let s = p.alloy.copper_pct + p.alloy.tin_pct + p.alloy.lead_pct
                + p.alloy.zinc_pct + p.alloy.other_impurities_pct;
            assert!((99.5..=100.5).contains(&s), "{did} alloy sum={s}");
        }
    }
}
