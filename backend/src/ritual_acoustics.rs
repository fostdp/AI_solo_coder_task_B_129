use crate::acoustic_fem_pool::AcousticFemPool;
use crate::config::AppConfig;
use crate::ethnic_comparator::EthnicComparator;
use crate::models::*;
use std::sync::Arc;

pub struct RitualAcoustics {
    config: Arc<AppConfig>,
    pool: Arc<AcousticFemPool>,
    ethnic: Arc<EthnicComparator>,
}

impl RitualAcoustics {
    pub fn new(
        config: Arc<AppConfig>,
        pool: Arc<AcousticFemPool>,
        ethnic: Arc<EthnicComparator>,
    ) -> Self {
        Self { config, pool, ethnic }
    }

    fn ac(&self) -> &crate::config::AcousticsConfig {
        self.config.acoustics.as_ref().unwrap()
    }

    pub fn ritual_sound_field(&self, req: RitualSoundFieldRequest) -> RitualSoundFieldResult {
        let observer_x = req.observer_x_m.unwrap_or(0.0);
        let observer_y = req.observer_y_m.unwrap_or(0.0);
        let observer_z = req.observer_z_m.unwrap_or(1.5);
        let radius = req.field_radius_m.unwrap_or(10.0);
        let grid_n = req.grid_resolution.unwrap_or(5).max(2);

        let placements: Vec<RitualDrumPlacement> = req.drum_placements;

        let drum_powers: Vec<f64> = placements.iter().map(|p| {
            self.ethnic.get_ethnic_drum_profile(&p.drum_id)
                .map(|d| self.pool.estimate_sound_power(&d) * p.relative_volume)
                .unwrap_or(0.1)
        }).collect();

        let drum_funds: Vec<f64> = placements.iter().map(|p| {
            self.ethnic.get_ethnic_drum_profile(&p.drum_id)
                .map(|d| self.pool.compute_modes_for_drum(&d).first().map(|(_, _, f, _)| *f).unwrap_or(80.0))
                .unwrap_or(80.0)
        }).collect();

        let mut sound_field = Vec::new();
        let step = 2.0 * radius / (grid_n as f64 - 1.0);
        for i in 0..grid_n {
            for j in 0..grid_n {
                let x = -radius + (i as f64) * step;
                let y = -radius + (j as f64) * step;
                let z = observer_z;
                let mut pressure_real_total = 0.0f64;
                let mut pressure_imag_total = 0.0f64;

                for (idx, placement) in placements.iter().enumerate() {
                    let dx = x - placement.position_x_m;
                    let dy = y - placement.position_y_m;
                    let dz = z - placement.position_z_m;
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.01);

                    let dx_mirror = x - placement.position_x_m;
                    let dy_mirror = y - placement.position_y_m;
                    let dz_mirror = z - (-placement.position_z_m);
                    let dist_mirror = (dx_mirror * dx_mirror + dy_mirror * dy_mirror + dz_mirror * dz_mirror).sqrt().max(0.01);

                    let freq = drum_funds[idx];
                    let air_absorption_db_per_km = 0.01 * freq / 1000.0 * 1.3;
                    let attenuation = (-air_absorption_db_per_km * dist / 8686.0).exp();
                    let attenuation_mirror = (-air_absorption_db_per_km * dist_mirror / 8686.0).exp();

                    let power = drum_powers[idx].max(1e-9);
                    let intensity = power / (4.0 * std::f64::consts::PI * dist * dist);
                    let p0 = (intensity * self.ac().air_density_kgm3 * self.ac().air_sound_speed_ms).sqrt() * attenuation;
                    let p_mirror_amp = p0 * 0.8 * attenuation_mirror;

                    let wavelength = self.ac().air_sound_speed_ms / freq.max(1.0);
                    let k = 2.0 * std::f64::consts::PI / wavelength.max(0.001);
                    let omega_t = 2.0 * std::f64::consts::PI * freq * placement.strike_phase_offset_s;

                    pressure_real_total += p0 * (k * dist - omega_t).cos();
                    pressure_imag_total += p0 * (k * dist - omega_t).sin();
                    pressure_real_total += p_mirror_amp * (k * dist_mirror - omega_t + std::f64::consts::PI).cos();
                    pressure_imag_total += p_mirror_amp * (k * dist_mirror - omega_t + std::f64::consts::PI).sin();
                }

                let pressure_pa = (pressure_real_total * pressure_real_total + pressure_imag_total * pressure_imag_total).sqrt();
                let pressure_pa_valid = pressure_pa.max(1e-12);
                let spl_db = 20.0 * (pressure_pa_valid / 2.0e-5).log10();
                let intensity = pressure_pa_valid * pressure_pa_valid
                    / (self.ac().air_density_kgm3 * self.ac().air_sound_speed_ms);

                sound_field.push(SoundFieldPoint {
                    x, y, z, pressure_pa: pressure_pa_valid, spl_db, intensity_wm2: intensity,
                });
            }
        }

        let observer_p = sound_field.iter()
            .min_by(|a, b| {
                let da = (a.x - observer_x) * (a.x - observer_x)
                    + (a.y - observer_y) * (a.y - observer_y)
                    + (a.z - observer_z) * (a.z - observer_z);
                let db = (b.x - observer_x) * (b.x - observer_x)
                    + (b.y - observer_y) * (b.y - observer_y)
                    + (b.z - observer_z) * (b.z - observer_z);
                da.partial_cmp(&db).unwrap()
            })
            .map(|p| p.spl_db).unwrap_or(80.0);

        let total_power: f64 = drum_powers.iter().sum();

        let mut envelope = Vec::new();
        let samples = 10usize;
        for i in 0..samples {
            let t = (i as f64) * 0.1;
            let mut amp = 0.0f64;
            for (idx, placement) in placements.iter().enumerate() {
                let dx = observer_x - placement.position_x_m;
                let dy = observer_y - placement.position_y_m;
                let dz = observer_z - placement.position_z_m;
                let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.01);
                let time_to_observer = dist / self.ac().air_sound_speed_ms;
                let elapsed = t - time_to_observer - placement.strike_phase_offset_s;
                if elapsed > 0.0 {
                    let power = drum_powers[idx].max(1e-9);
                    let intensity = power / (4.0 * std::f64::consts::PI * dist * dist);
                    let decay = (-elapsed * 0.5).exp();
                    amp += (intensity * self.ac().air_density_kgm3 * self.ac().air_sound_speed_ms).sqrt() * decay;
                }
            }
            envelope.push((t, amp));
        }

        let scene = self.infer_ritual_scene(&placements);
        RitualSoundFieldResult {
            sound_field,
            observer_spl_db: observer_p,
            total_radiated_power_w: total_power,
            drum_count: placements.len(),
            temporal_envelope: envelope,
            ritual_scene: scene,
        }
    }

    fn infer_ritual_scene(&self, placements: &[RitualDrumPlacement]) -> RitualSceneInfo {
        let ethnic_groups: Vec<String> = placements.iter().filter_map(|p| {
            self.ethnic.get_ethnic_drum_profile(&p.drum_id)
                .map(|d| d.ethnic_group.clone())
        }).collect();
        let n = placements.len();

        if n <= 1 {
            RitualSceneInfo {
                scene_type: "单鼓独奏仪式".to_string(),
                description: "一面铜鼓独奏，常用于小型祭祀或家庭祈福。".to_string(),
                historical_reference: "古代民间家用祭祀，如请神、安宅、送魂等。".to_string(),
            }
        } else if n <= 3 {
            let dominant = ethnic_groups.iter().cloned().next().unwrap_or_else(|| "多民族".into());
            RitualSceneInfo {
                scene_type: format!("{dominant} 小型铜鼓组"),
                description: "2~3面铜鼓齐奏，常见于村社级祭祀活动。".to_string(),
                historical_reference: "壮族雷王祭、苗族牯藏节中的小鼓组形式。".to_string(),
            }
        } else {
            let unique: std::collections::HashSet<_> = ethnic_groups.iter().collect();
            if unique.len() > 1 {
                RitualSceneInfo {
                    scene_type: "多民族大型联合祭祀".to_string(),
                    description: "多民族铜鼓联合演奏，场面宏大，气势雄浑。".to_string(),
                    historical_reference: "桂西三月三、滇东铜鼓节等跨民族文化盛会。".to_string(),
                }
            } else {
                RitualSceneInfo {
                    scene_type: format!("{} 大型铜鼓阵", ethnic_groups[0]),
                    description: "同民族多面铜鼓组成鼓阵，整齐划一，震撼人心。".to_string(),
                    historical_reference: "东兰壮族铜鼓节、瑶族盘王节大型鼓阵。".to_string(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> RitualAcoustics {
        let config = Arc::new(crate::config::AppConfig::load());
        let pool = Arc::new(AcousticFemPool::new(config.clone()));
        let ethnic = Arc::new(EthnicComparator::new(config.clone(), pool.clone()));
        RitualAcoustics::new(config, pool, ethnic)
    }

    #[test]
    fn test_sound_field_spl_gradient() {
        let r = make();
        let placements = vec![
            RitualDrumPlacement {
                drum_id: "zhuang-dagu".into(),
                position_x_m: 2.0, position_y_m: 0.0, position_z_m: 1.2,
                relative_volume: 1.0, strike_phase_offset_s: 0.0,
            },
        ];
        let res = r.ritual_sound_field(RitualSoundFieldRequest {
            drum_placements: placements,
            observer_x_m: Some(2.0), observer_y_m: Some(0.0), observer_z_m: Some(1.5),
            field_radius_m: Some(5.0), grid_resolution: Some(4),
        });
        let spls: Vec<f64> = res.sound_field.iter().map(|p| p.spl_db).collect();
        let max = spls.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = spls.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(max > min, "SPL gradient max({max}) > min({min})");
    }

    #[test]
    fn test_temporal_envelope_decays() {
        let r = make();
        let placements = vec![
            RitualDrumPlacement {
                drum_id: "zhuang-dagu".into(),
                position_x_m: 1.0, position_y_m: 0.0, position_z_m: 1.2,
                relative_volume: 1.0, strike_phase_offset_s: 0.0,
            },
        ];
        let res = r.ritual_sound_field(RitualSoundFieldRequest {
            drum_placements: placements,
            observer_x_m: Some(1.0), observer_y_m: Some(0.0), observer_z_m: Some(1.5),
            field_radius_m: Some(3.0), grid_resolution: Some(3),
        });
        let amps: Vec<f64> = res.temporal_envelope.iter().map(|(_, a)| *a).collect();
        let max_amp = amps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let last_amp = amps.last().copied().unwrap_or(0.0);
        assert!(max_amp > last_amp || max_amp <= 1e-9, "envelope max({max_amp}) >= last({last_amp})");
    }

    #[test]
    fn test_observer_at_drum_no_nan_inf() {
        let r = make();
        let placements = vec![
            RitualDrumPlacement {
                drum_id: "zhuang-dagu".into(),
                position_x_m: 0.0, position_y_m: 0.0, position_z_m: 1.2,
                relative_volume: 1.0, strike_phase_offset_s: 0.0,
            },
        ];
        let res = r.ritual_sound_field(RitualSoundFieldRequest {
            drum_placements: placements,
            observer_x_m: Some(0.0), observer_y_m: Some(0.0), observer_z_m: Some(1.2),
            field_radius_m: Some(2.0), grid_resolution: Some(3),
        });
        for p in &res.sound_field {
            assert!(!p.spl_db.is_nan() && !p.spl_db.is_infinite());
            assert!(!p.pressure_pa.is_nan() && !p.pressure_pa.is_infinite());
        }
        assert!(!res.observer_spl_db.is_nan() && !res.observer_spl_db.is_infinite());
    }

    #[test]
    fn test_empty_placements_default() {
        let r = make();
        let res = r.ritual_sound_field(RitualSoundFieldRequest {
            drum_placements: vec![],
            observer_x_m: None, observer_y_m: None, observer_z_m: None,
            field_radius_m: None, grid_resolution: None,
        });
        assert_eq!(res.drum_count, 0);
        assert!(!res.sound_field.is_empty());
        for p in &res.sound_field {
            assert!(p.spl_db.is_finite());
            assert!(p.pressure_pa.is_finite());
        }
    }
}
