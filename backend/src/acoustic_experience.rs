use crate::config::AppConfig;
use crate::models::*;
use std::sync::Arc;

pub struct AcousticExperienceService {
    config: Arc<AppConfig>,
}

impl AcousticExperienceService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    fn ac(&self) -> &crate::config::AcousticsConfig {
        self.config.acoustics.as_ref().unwrap()
    }

    pub fn get_ethnic_drum_library(&self) -> Vec<EthnicDrumLibraryEntry> {
        vec![
            EthnicDrumLibraryEntry {
                drum_id: "zhuang-dagu".to_string(),
                name: "壮族大铜鼓".to_string(),
                ethnic_group: "壮族".to_string(),
                diameter_cm: 100.0,
                fundamental_hz: 98.0,
                cultural_tag: "蛙图腾·雷王鼓".to_string(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "zhuang-magu".to_string(),
                name: "壮族麻江型铜鼓".to_string(),
                ethnic_group: "壮族".to_string(),
                diameter_cm: 50.0,
                fundamental_hz: 210.0,
                cultural_tag: "丧葬·祭祀".to_string(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "miao-dagu".to_string(),
                name: "苗族大铜鼓".to_string(),
                ethnic_group: "苗族".to_string(),
                diameter_cm: 80.0,
                fundamental_hz: 130.0,
                cultural_tag: "牯藏节·祭祖".to_string(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "miao-xiaogu".to_string(),
                name: "苗族小铜鼓".to_string(),
                ethnic_group: "苗族".to_string(),
                diameter_cm: 35.0,
                fundamental_hz: 310.0,
                cultural_tag: "跳月·芦笙舞".to_string(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "yao-dagu".to_string(),
                name: "瑶族大铜鼓".to_string(),
                ethnic_group: "瑶族".to_string(),
                diameter_cm: 70.0,
                fundamental_hz: 150.0,
                cultural_tag: "盘王节·长鼓舞".to_string(),
            },
            EthnicDrumLibraryEntry {
                drum_id: "yao-guzai".to_string(),
                name: "瑶族铜鼓仔".to_string(),
                ethnic_group: "瑶族".to_string(),
                diameter_cm: 25.0,
                fundamental_hz: 440.0,
                cultural_tag: "耍歌堂·喜庆".to_string(),
            },
        ]
    }

    pub fn get_ethnic_drum_profile(&self, drum_id: &str) -> Option<EthnicDrumProfile> {
        let library = self.get_ethnic_drum_library();
        let entry = library.iter().find(|e| e.drum_id == drum_id)?;

        let (alloy, thickness_profile, avg_thickness, cultural, uses) = match drum_id {
            "zhuang-dagu" => (
                AlloyComposition {
                    copper_pct: 82.0, tin_pct: 12.0, lead_pct: 4.5, zinc_pct: 0.8, other_impurities_pct: 0.7,
                },
                "central_thick".to_string(),
                4.5,
                "壮族先民视铜鼓为雷王化身，蛙纹象征雨水与丰收。".to_string(),
                vec!["祭祀雷王".to_string(), "祈雨仪式".to_string(), "丰收庆典".to_string(), "丧葬礼仪".to_string()],
            ),
            "zhuang-magu" => (
                AlloyComposition {
                    copper_pct: 80.0, tin_pct: 14.0, lead_pct: 4.0, zinc_pct: 1.0, other_impurities_pct: 1.0,
                },
                "uniform".to_string(),
                3.2,
                "麻江型铜鼓是广西壮族地区最常见的类型，纹饰精美。".to_string(),
                vec!["民间祭祀".to_string(), "婚丧嫁娶".to_string(), "节日庆典".to_string(), "铜鼓舞".to_string()],
            ),
            "miao-dagu" => (
                AlloyComposition {
                    copper_pct: 84.0, tin_pct: 10.0, lead_pct: 4.0, zinc_pct: 0.5, other_impurities_pct: 1.5,
                },
                "edge_thick".to_string(),
                5.0,
                "苗族铜鼓是牯藏节的核心礼器，每十三年举办一次。".to_string(),
                vec!["牯藏大典".to_string(), "祭祖仪式".to_string(), "丧葬送魂".to_string(), "芦笙伴奏".to_string()],
            ),
            "miao-xiaogu" => (
                AlloyComposition {
                    copper_pct: 85.0, tin_pct: 11.0, lead_pct: 2.5, zinc_pct: 0.3, other_impurities_pct: 1.2,
                },
                "uniform".to_string(),
                2.5,
                "小型铜鼓便于携带，是苗族跳月活动的重要乐器。".to_string(),
                vec!["跳月舞".to_string(), "青年社交".to_string(), "节日娱乐".to_string(), "芦笙合奏".to_string()],
            ),
            "yao-dagu" => (
                AlloyComposition {
                    copper_pct: 78.0, tin_pct: 15.0, lead_pct: 5.0, zinc_pct: 0.5, other_impurities_pct: 1.5,
                },
                "central_thick".to_string(),
                4.8,
                "瑶族铜鼓与长鼓并称，盘王节中铜鼓指挥长鼓节奏。".to_string(),
                vec!["盘王节祭祀".to_string(), "长鼓舞伴奏".to_string(), "耍歌堂".to_string(), "驱邪仪式".to_string()],
            ),
            "yao-guzai" => (
                AlloyComposition {
                    copper_pct: 83.0, tin_pct: 12.0, lead_pct: 3.5, zinc_pct: 0.5, other_impurities_pct: 1.0,
                },
                "uniform".to_string(),
                2.0,
                "铜鼓仔音色明亮清脆，是瑶族儿童游戏和青年对歌的伴奏乐器。".to_string(),
                vec!["喜庆节日".to_string(), "歌堂伴奏".to_string(), "儿童游戏".to_string(), "信号传递".to_string()],
            ),
            _ => return None,
        };

        let (origin_region, estimated_era, height_cm, mass_kg) = match drum_id {
            "zhuang-dagu" => ("广西右江流域".to_string(), "汉代-唐代".to_string(), 60.0, 80.0),
            "zhuang-magu" => ("广西麻江县".to_string(), "宋代-明代".to_string(), 30.0, 18.0),
            "miao-dagu" => ("贵州黔东南".to_string(), "明代-清代".to_string(), 45.0, 45.0),
            "miao-xiaogu" => ("贵州雷山".to_string(), "清代-近代".to_string(), 20.0, 8.0),
            "yao-dagu" => ("广西金秀".to_string(), "清代".to_string(), 40.0, 35.0),
            "yao-guzai" => ("广东连南".to_string(), "近代".to_string(), 15.0, 4.0),
            _ => ("未知".to_string(), "未知".to_string(), 30.0, 20.0),
        };

        Some(EthnicDrumProfile {
            drum_id: entry.drum_id.clone(),
            name: entry.name.clone(),
            ethnic_group: entry.ethnic_group.clone(),
            origin_region,
            estimated_era,
            diameter_cm: entry.diameter_cm,
            height_cm,
            mass_kg,
            alloy,
            thickness_profile,
            avg_thickness_mm: avg_thickness,
            cultural_significance: cultural,
            traditional_uses: uses,
        })
    }

    fn compute_modes_for_drum(&self, drum: &EthnicDrumProfile) -> Vec<(usize, usize, f64, f64)> {
        let ac = self.ac();
        let a = drum.diameter_cm / 200.0;
        let h = drum.avg_thickness_mm / 1000.0;
        let rho = 8700.0_f64;
        let nu = 0.34_f64;
        let e = 1.1e11_f64;
        let d = e * h.powi(3) / (12.0 * (1.0 - nu.powi(2)));

        ac.eigenvalues_lambda
            .iter()
            .filter(|e| e.m <= 4 && e.n <= 4)
            .map(|e| {
                let f = e.lambda.powi(2) / (2.0 * std::f64::consts::PI * a.powi(2))
                    * (d / (rho * h)).sqrt();
                let damping = 0.005 + (e.n as f64) * 0.002;
                (e.m, e.n, f, damping)
            })
            .take(15)
            .collect()
    }

    fn compute_spectrum_for_drum(&self, drum: &EthnicDrumProfile) -> Vec<SpectrumBin> {
        let modes = self.compute_modes_for_drum(drum);
        let mut spectrum = Vec::new();
        let max_f = 2000.0;
        let bin_size = 5.0;
        let num_bins = (max_f / bin_size) as usize;

        for i in 0..num_bins {
            let center_freq = (i as f64 + 0.5) * bin_size;
            let mut amplitude = 0.0;

            for (m, n, f, _damp) in &modes {
                let width = f * 0.02 + 3.0;
                let diff = (center_freq - f).abs();
                if diff < width * 3.0 {
                    let gaussian = (- (diff * diff) / (2.0 * width * width)).exp();
                    let mode_amp = 1.0 / ((*m as f64 + 1.0) * (*n as f64 + 1.0)).sqrt();
                    amplitude += gaussian * mode_amp;
                }
            }

            spectrum.push(SpectrumBin {
                frequency_hz: center_freq,
                amplitude_db: if amplitude > 0.0 {
                    20.0 * amplitude.log10() + 80.0
                } else {
                    0.0
                },
            });
        }

        spectrum
    }

    fn estimate_sound_power(&self, drum: &EthnicDrumProfile) -> f64 {
        let modes = self.compute_modes_for_drum(drum);
        let a = drum.diameter_cm / 200.0;
        let c0 = self.ac().air_sound_speed_ms;
        let mut total = 0.0;

        for (_m, _n, f, _damp) in modes.iter().take(5) {
            let ka = 2.0 * std::f64::consts::PI * f * a / c0;
            let sigma = if ka < 0.5 {
                ka * ka * 0.8
            } else if ka < 5.0 {
                1.0 - (-ka).exp()
            } else {
                1.0
            };
            total += sigma * 0.1;
        }

        total
    }

    pub fn compare_ethnic_drums(&self, drum_ids: Vec<String>) -> EthnicAcousticComparison {
        let drums: Vec<EthnicDrumProfile> = drum_ids
            .iter()
            .filter_map(|id| self.get_ethnic_drum_profile(id))
            .collect();

        let frequency_spectra: Vec<(String, Vec<SpectrumBin>)> = drums
            .iter()
            .map(|drum| {
                let spectrum = self.compute_spectrum_for_drum(drum);
                (drum.drum_id.clone(), spectrum)
            })
            .collect();

        let modes_comparison: Vec<(String, Vec<VibrationMode>)> = drums
            .iter()
            .map(|drum| {
                let modes_raw = self.compute_modes_for_drum(drum);
                let modes: Vec<VibrationMode> = modes_raw
                    .iter()
                    .enumerate()
                    .map(|(idx, (m, n, f, damp))| VibrationMode {
                        mode_order: idx + 1,
                        frequency_hz: *f,
                        nonlinear_frequency_hz: *f * 1.02,
                        damping_ratio: *damp,
                        node_pattern: format!("m={},n={}", m, n),
                        modal_displacements: vec![],
                        effective_force_amplitude: 1.0 / ((idx + 1) as f64).sqrt(),
                        arc_length_converged: true,
                    })
                    .collect();
                (drum.drum_id.clone(), modes)
            })
            .collect();

        let comparison_metrics = self.compute_comparison_metrics(&drums);

        EthnicAcousticComparison {
            drums,
            comparison_metrics,
            frequency_spectra,
            vibration_modes_comparison: modes_comparison,
        }
    }

    fn compute_comparison_metrics(&self, drums: &[EthnicDrumProfile]) -> EthnicComparisonMetrics {
        let mut fundamental_freqs = Vec::new();
        let mut sound_quality_scores = Vec::new();
        let mut radiated_powers = Vec::new();
        let mut harmonic_series = Vec::new();
        let mut decay_times_s = Vec::new();
        let mut brightness = Vec::new();

        for drum in drums {
            let modes = self.compute_modes_for_drum(drum);
            let fund_freq = modes.first().map(|(_, _, f, _)| *f).unwrap_or(100.0);

            let quality = 0.7 + (drum.avg_thickness_mm / 10.0).min(0.25);
            let power = self.estimate_sound_power(drum);

            let harmonics: Vec<f64> = modes.iter().take(6).map(|(_, _, f, _)| *f).collect();
            let decay = 2.0 + (drum.diameter_cm / 100.0);

            let bright = if modes.len() >= 3 {
                let hi = modes[2].2;
                let lo = modes[0].2;
                hi / lo
            } else {
                2.0
            };

            fundamental_freqs.push((drum.drum_id.clone(), fund_freq));
            sound_quality_scores.push((drum.drum_id.clone(), quality));
            radiated_powers.push((drum.drum_id.clone(), power));
            harmonic_series.push((drum.drum_id.clone(), harmonics));
            decay_times_s.push((drum.drum_id.clone(), decay));
            brightness.push((drum.drum_id.clone(), bright));
        }

        EthnicComparisonMetrics {
            fundamental_freqs,
            sound_quality_scores,
            radiated_powers,
            harmonic_series,
            decay_times_s,
            brightness,
        }
    }

    pub fn get_timpani_profile(&self, size_inches: f64) -> TimpaniDrumProfile {
        let diameter_cm = size_inches * 2.54;
        let fundamental = 523.25 * (29.0 / size_inches).sqrt();
        let freq_range = (fundamental * 0.75, fundamental * 1.33);

        TimpaniDrumProfile {
            name: format!("{}\" 定音鼓", size_inches),
            diameter_inches: size_inches,
            diameter_cm,
            fundamental_freq_hz: fundamental,
            freq_range_hz: freq_range,
            material: "聚酯薄膜鼓皮+铜制共鸣锅".to_string(),
            tension_pascals: 8000.0,
            damping_ratio: 0.01,
            harmonic_structure: vec![1.0, 1.59, 2.0, 2.45, 2.9, 3.4],
        }
    }

    pub fn cross_era_comparison(
        &self,
        drum_id: &str,
        timpani_size_inches: Option<f64>,
    ) -> Option<CrossEraComparison> {
        let ancient = self.get_ethnic_drum_profile(drum_id)?;
        let timpani_size = timpani_size_inches.unwrap_or(29.0);
        let modern = self.get_timpani_profile(timpani_size);

        let ancient_spectrum = self.compute_spectrum_for_drum(&ancient);
        let ancient_modes = self.compute_modes_for_drum(&ancient);
        let ancient_fund = ancient_modes.first().map(|(_, _, f, _)| *f).unwrap_or(100.0);
        let ancient_harm_ratios: Vec<f64> = ancient_modes
            .iter()
            .take(6)
            .map(|(_, _, f, _)| f / ancient_fund)
            .collect();

        let ancient_power = self.estimate_sound_power(&ancient);

        let modern_spectrum = self.compute_timpani_spectrum(&modern);
        let modern_harm_ratios = modern.harmonic_structure.clone();

        let metrics = CrossEraMetrics {
            ancient_fundamental_hz: ancient_fund,
            modern_fundamental_hz: modern.fundamental_freq_hz,
            ancient_harmonic_ratios: ancient_harm_ratios,
            modern_harmonic_ratios: modern_harm_ratios.clone(),
            ancient_decay_s: 2.5 + ancient.diameter_cm / 100.0,
            modern_decay_s: 1.2,
            ancient_brightness: (ancient_modes.get(2).map(|m| m.2).unwrap_or(300.0) / ancient_fund) * 30.0,
            modern_brightness: 85.0,
            ancient_radiated_power_w: ancient_power,
            modern_radiated_power_w: 0.5,
            pitch_tunability: "铜鼓：固定音高，由铸造时的尺寸和壁厚决定；定音鼓：可调音高，通过踏板改变鼓皮张力实现".to_string(),
            cultural_context: "古代铜鼓是礼器与乐器的统一体，沟通天地人神，承载着民族的宇宙观和宗教信仰；现代定音鼓是交响乐队的核心打击乐器，追求精准的音高控制与音色融合，服务于专业音乐表演。".to_string(),
        };

        Some(CrossEraComparison {
            ancient_drum: ancient,
            modern_drum: modern,
            metrics_comparison: metrics,
            ancient_spectrum,
            modern_spectrum,
        })
    }

    fn compute_timpani_spectrum(&self, timpani: &TimpaniDrumProfile) -> Vec<SpectrumBin> {
        let mut spectrum = Vec::new();
        let max_f = 3000.0;
        let bin_size = 5.0;
        let num_bins = (max_f / bin_size) as usize;

        for i in 0..num_bins {
            let center_freq = (i as f64 + 0.5) * bin_size;
            let mut amplitude = 0.0;

            for (idx, &ratio) in timpani.harmonic_structure.iter().enumerate() {
                let f = timpani.fundamental_freq_hz * ratio;
                let width = f * 0.01 + 2.0;
                let diff = (center_freq - f).abs();
                if diff < width * 3.0 {
                    let gaussian = (- (diff * diff) / (2.0 * width * width)).exp();
                    let mode_amp = 1.0 / ((idx + 1) as f64).sqrt();
                    amplitude += gaussian * mode_amp;
                }
            }

            spectrum.push(SpectrumBin {
                frequency_hz: center_freq,
                amplitude_db: if amplitude > 0.0 {
                    20.0 * amplitude.log10() + 85.0
                } else {
                    0.0
                },
            });
        }

        spectrum
    }

    pub fn ritual_sound_field(&self, req: RitualSoundFieldRequest) -> RitualSoundFieldResult {
        let field_radius = req.field_radius_m.unwrap_or(15.0);
        let resolution = req.grid_resolution.unwrap_or(12);

        let observer_x = req.observer_x_m.unwrap_or(0.0);
        let observer_y = req.observer_y_m.unwrap_or(0.0);
        let observer_z = req.observer_z_m.unwrap_or(1.5);

        let mut field_points = Vec::new();

        for theta_idx in 0..resolution {
            let theta = (theta_idx as f64 / resolution as f64) * std::f64::consts::PI * 2.0;
            for phi_idx in 0..=resolution / 2 {
                let phi = (phi_idx as f64 / (resolution / 2) as f64) * std::f64::consts::PI / 2.0;

                let x = field_radius * phi.cos() * theta.cos();
                let y = field_radius * phi.cos() * theta.sin();
                let z = field_radius * phi.sin();

                let mut total_intensity = 0.0f64;

                for placement in &req.drum_placements {
                    let dx = x - placement.position_x_m;
                    let dy = y - placement.position_y_m;
                    let dz = z - placement.position_z_m;
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.5);

                    let profile = self.get_ethnic_drum_profile(&placement.drum_id)
                        .unwrap_or_else(|| self.get_ethnic_drum_profile("zhuang-dagu").unwrap());
                    let power = self.estimate_sound_power(&profile) * placement.relative_volume;

                    let intensity = power / (4.0 * std::f64::consts::PI * dist * dist);
                    total_intensity += intensity;
                }

                let p_ref = 2e-5;
                let spl = if total_intensity > 1e-12 {
                    let p_rms = (total_intensity * self.ac().air_density_kgm3 * self.ac().air_sound_speed_ms).sqrt();
                    20.0 * (p_rms / p_ref).log10()
                } else {
                    0.0
                };

                let pressure = if spl > 0.0 {
                    p_ref * (spl / 20.0).exp()
                } else {
                    0.0
                };

                field_points.push(SoundFieldPoint {
                    x,
                    y,
                    z,
                    pressure_pa: pressure,
                    spl_db: spl,
                    intensity_wm2: total_intensity,
                });
            }
        }

        let mut observer_spl = 0.0;
        if !req.drum_placements.is_empty() {
            let mut observer_intensity = 0.0;
            for placement in &req.drum_placements {
                let dx = observer_x - placement.position_x_m;
                let dy = observer_y - placement.position_y_m;
                let dz = observer_z - placement.position_z_m;
                let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.5);

                let profile = self.get_ethnic_drum_profile(&placement.drum_id)
                    .unwrap_or_else(|| self.get_ethnic_drum_profile("zhuang-dagu").unwrap());
                let power = self.estimate_sound_power(&profile) * placement.relative_volume;

                observer_intensity += power / (4.0 * std::f64::consts::PI * dist * dist);
            }
            if observer_intensity > 1e-12 {
                let p_rms = (observer_intensity * self.ac().air_density_kgm3 * self.ac().air_sound_speed_ms).sqrt();
                observer_spl = 20.0 * (p_rms / 2e-5).log10();
            }
        }

        let total_power: f64 = req.drum_placements
            .iter()
            .map(|p| {
                let profile = self.get_ethnic_drum_profile(&p.drum_id)
                    .unwrap_or_else(|| self.get_ethnic_drum_profile("zhuang-dagu").unwrap());
                self.estimate_sound_power(&profile) * p.relative_volume
            })
            .sum();

        let mut envelope = Vec::new();
        for t in 0..100 {
            let time = t as f64 * 0.03;
            let mut amp = 0.0;
            for placement in &req.drum_placements {
                let phase = placement.strike_phase_offset_s;
                let dt = time - phase;
                if dt >= 0.0 {
                    let env = (-dt / 2.0).exp() * (1.0 - (-dt / 0.01).exp());
                    amp += env * placement.relative_volume;
                }
            }
            envelope.push((time, amp));
        }

        let scene_info = self.infer_ritual_scene(&req.drum_placements);

        RitualSoundFieldResult {
            sound_field: field_points,
            observer_spl_db: observer_spl,
            total_radiated_power_w: total_power,
            drum_count: req.drum_placements.len(),
            temporal_envelope: envelope,
            ritual_scene: scene_info,
        }
    }

    fn infer_ritual_scene(&self, placements: &[RitualDrumPlacement]) -> RitualSceneInfo {
        let count = placements.len();

        if count == 0 {
            return RitualSceneInfo {
                scene_type: "无鼓".to_string(),
                description: "请添加铜鼓开始仪式声场模拟".to_string(),
                historical_reference: "".to_string(),
            };
        }

        let unique_ethnic_groups: std::collections::HashSet<String> = placements
            .iter()
            .filter_map(|p| {
                self.get_ethnic_drum_profile(&p.drum_id)
                    .map(|d| d.ethnic_group)
            })
            .collect();

        let (scene_type, description, reference) = if count == 1 {
            (
                "独奏祭祀".to_string(),
                "单鼓独奏是最古老的铜鼓使用形式，通常由巫师或族长在祭祀中敲击，沟通神灵。".to_string(),
                "《后汉书·马援传》记载：骆越之民，好相攻击，得铜鼓，鼓高大者为贵，群情慑服。".to_string(),
            )
        } else if count <= 4 {
            (
                "四方祭祀阵".to_string(),
                "四面铜鼓按东南西北四方布阵，象征四象四季，是中等规模祭祀的典型配置。".to_string(),
                "壮族《布洛陀经诗》载：铜鼓镇四方，蛙鸣唤雨水。".to_string(),
            )
        } else if count <= 8 {
            (
                "八方祭祀阵".to_string(),
                "八面对称排列的铜鼓大阵，用于重大祭祀仪式，声音可达十里之外。".to_string(),
                "《唐书·南蛮传》：蛮人宴会，击铜鼓，吹角，歌舞为乐。".to_string(),
            )
        } else {
            (
                "百鼓齐鸣大典".to_string(),
                "数十面上百面铜鼓同时敲击，是民族地区最盛大的节庆场景，震撼人心。".to_string(),
                "广西东兰县每年蚂拐节有百鼓齐鸣的传统，纪念青蛙神，祈求风调雨顺。".to_string(),
            )
        };

        let ethnic_str = if unique_ethnic_groups.len() > 1 {
            format!("多民族（{}个）", unique_ethnic_groups.len())
        } else {
            unique_ethnic_groups.into_iter().next().unwrap_or_else(|| "未知".to_string())
        };

        RitualSceneInfo {
            scene_type: format!("{}·{}", ethnic_str, scene_type),
            description,
            historical_reference: reference,
        }
    }

    pub fn virtual_tap(&self, req: VirtualTapRequest) -> Option<VirtualTapResult> {
        let drum = self.get_ethnic_drum_profile(&req.drum_id)?;

        let x = (req.x_frac - 0.5) * 2.0;
        let y = (req.y_frac - 0.5) * 2.0;
        let r_frac = (x * x + y * y).sqrt().clamp(0.0, 0.95);

        let zone_name = if r_frac < 0.3 {
            "鼓心（太阳纹）".to_string()
        } else if r_frac < 0.6 {
            "鼓面中部".to_string()
        } else if r_frac < 0.85 {
            "鼓面边缘".to_string()
        } else {
            "鼓边（禁止敲击）".to_string()
        };

        let modes = self.compute_modes_for_drum(&drum);
        let _a = drum.diameter_cm / 200.0;

        let mut excitation_weights: Vec<f64> = modes
            .iter()
            .map(|(m, n, _f, _damp)| {
                let r_norm = r_frac;

                let radial_weight = match (*m, *n) {
                    (0, 1) => (1.0 - r_norm * r_norm).max(0.0),
                    (0, _) => (1.0 - r_norm * r_norm).powi(*n as i32).max(0.0),
                    (m_val, 1) => r_norm.powi(m_val as i32) * (1.0 - r_norm * r_norm).max(0.0),
                    (m_val, n_val) => {
                        r_norm.powi(m_val as i32) * (1.0 - r_norm * r_norm).powi(n_val as i32).max(0.0)
                    }
                };

                let center_bias = if *m == 0 && *n == 1 {
                    1.5
                } else if *m >= 2 {
                    0.6
                } else {
                    1.0
                };

                let edge_damping = (1.0 - r_norm * 0.8).max(0.1);
                let strike_eff = radial_weight * edge_damping * req.strike_force * center_bias;

                if strike_eff.is_nan() || strike_eff.is_infinite() { 0.0 } else { strike_eff }
            })
            .collect();

        let max_w = excitation_weights.iter().cloned().fold(0.0f64, f64::max);
        if max_w > 0.0 {
            for w in &mut excitation_weights {
                *w /= max_w;
            }
        }

        let spectrum = Self::synthesize_spectrum(&modes, &excitation_weights);

        let fund_freq = modes.first().map(|(_, _, f, _)| *f).unwrap_or(100.0);

        let harmonics: Vec<(f64, f64)> = modes
            .iter()
            .zip(excitation_weights.iter())
            .take(8)
            .map(|((_, _, f, _), weight)| (*f, *weight))
            .collect();

        let decay_time = 1.5 + (1.0 - r_frac) * 1.5;

        let mut total_hi = 0.0;
        let mut total_lo = 0.0;
        for ((_, _, f, _), weight) in modes.iter().zip(excitation_weights.iter()).take(10) {
            if *f > fund_freq * 2.0 {
                total_hi += weight;
            } else {
                total_lo += weight;
            }
        }
        let brightness = if total_lo > 1e-6 { (total_hi / total_lo * 50.0).min(100.0) } else { 50.0 };

        let timbre_character = if r_frac < 0.3 {
            "浑厚深沉，基音强，余韵悠长".to_string()
        } else if r_frac < 0.6 {
            "平衡饱满，高中低频兼备，是最常用的敲击位置".to_string()
        } else if r_frac < 0.85 {
            "明亮清脆，泛音丰富，穿透力强".to_string()
        } else {
            "金属撞击声，短促刺耳".to_string()
        };

        let attack = 0.005 + r_frac * 0.01;
        let decay = decay_time * 0.3;
        let sustain = 0.3;
        let release = decay_time * 0.7;

        let harmonic_gains: Vec<f64> = excitation_weights.iter().take(10).cloned().collect();

        let filter_cutoff = 500.0 + r_frac * 3000.0;
        let filter_q = 1.0 + (1.0 - r_frac) * 2.0;

        let overall_gain = (0.3 + r_frac * 0.7) * req.strike_force.clamp(0.1, 2.0);

        let mut envelope = Vec::new();
        for t in 0..100 {
            let time = t as f64 * 0.03;
            let env = if time < attack {
                time / attack
            } else if time < attack + decay {
                1.0 - (1.0 - sustain) * (time - attack) / decay
            } else {
                sustain * (-(time - attack - decay) / release).exp()
            };
            envelope.push((time, env));
        }

        Some(VirtualTapResult {
            spectrum,
            fundamental_freq_hz: fund_freq,
            harmonics,
            amplitude_envelope: envelope,
            decay_time_s: decay_time,
            brightness,
            timbre_character,
            zone_name,
            web_audio_params: WebAudioParams {
                base_freq: fund_freq,
                harmonic_gains,
                attack_s: attack,
                decay_s: decay,
                sustain_level: sustain,
                release_s: release,
                filter_cutoff_hz: filter_cutoff,
                filter_q: filter_q,
                overall_gain,
            },
        })
    }

    fn get_lambda(&self, m: usize, n: usize) -> f64 {
        self.ac().eigenvalues_lambda
            .iter()
            .find(|e| e.m == m && e.n == n)
            .map(|e| e.lambda)
            .unwrap_or_else(|| {
                (n as f64 + m as f64 * 0.5) * std::f64::consts::PI
            })
    }

    fn synthesize_spectrum(
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

    fn bessel_j(m: i32, x: f64) -> f64 {
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

    fn bessel_i(m: i32, x: f64) -> f64 {
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

    fn factorial(n: f64) -> f64 {
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
        assert!((1.0..=100.0).contains(&c.brightness),
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
}
