use crate::acoustic_fem_pool::AcousticFemPool;
use crate::config::AppConfig;
use crate::ethnic_comparator::EthnicComparator;
use crate::models::*;
use std::sync::Arc;

pub struct VrDrumEnsemble {
    config: Arc<AppConfig>,
    pool: Arc<AcousticFemPool>,
    ethnic: Arc<EthnicComparator>,
}

impl VrDrumEnsemble {
    pub fn new(
        config: Arc<AppConfig>,
        pool: Arc<AcousticFemPool>,
        ethnic: Arc<EthnicComparator>,
    ) -> Self {
        Self { config, pool, ethnic }
    }

    pub fn virtual_tap(&self, req: VirtualTapRequest) -> Option<VirtualTapResult> {
        let drum = self.ethnic.get_ethnic_drum_profile(&req.drum_id)?;
        let modes = self.pool.compute_modes_for_drum(&drum);
        let spectrum = self.pool.compute_spectrum_for_drum(&drum);
        let fund = modes.first().map(|(_, _, f, _)| *f).unwrap_or(80.0);

        let dx = req.x_frac - 0.5;
        let dy = req.y_frac - 0.5;
        let radius_frac = (dx * dx + dy * dy).sqrt();
        let (zone_name, zone_amp, brightness_factor) = if radius_frac < 0.2 {
            ("鼓心".to_string(), 1.0, 0.5)
        } else if radius_frac < 0.5 {
            ("鼓中".to_string(), 0.85, 0.7)
        } else {
            ("鼓边".to_string(), 0.65, 0.9)
        };

        let harmonics: Vec<(f64, f64)> = modes
            .iter()
            .take(10)
            .map(|(m, _n, f, _d)| {
                let mode_factor = if *m == 0 { zone_amp } else { zone_amp * 0.7 };
                (*f, mode_factor * req.strike_force.clamp(0.1, 5.0))
            })
            .collect();

        let avg_damping: f64 = modes.iter()
            .map(|(_, _, _, d)| *d).sum::<f64>() / modes.len().max(1) as f64;
        let decay_time = (if avg_damping > 0.0 { 1.0 / avg_damping } else { 2.0 }).clamp(0.05, 29.9);

        let bright_count = modes.iter()
            .filter(|(_, _, f, _)| *f > fund * 2.0).count() as f64;
        let brightness = (1.0 + bright_count * 40.0 * brightness_factor).clamp(1.0, 499.9);

        let timbre = match zone_name.as_str() {
            "鼓心" => "深沉浑厚·共鸣绵长·典型祭祀音",
            "鼓中" => "平衡明亮·清晰可辨·日常演奏音",
            _ => "尖锐清脆·富有穿透力·信号音",
        }.to_string();

        let harmonic_gains: Vec<f64> = harmonics.iter().take(8).map(|(_, a)| *a).collect();
        let strike_factor = req.strike_force.clamp(0.1, 5.0) / 2.5;
        let web_audio = WebAudioParams {
            base_freq: fund,
            harmonic_gains,
            attack_s: 0.002 + 0.005 / strike_factor.max(0.2),
            decay_s: 0.08 * decay_time.min(5.0),
            sustain_level: 0.35,
            release_s: 0.25 * decay_time.min(8.0),
            filter_cutoff_hz: (fund * 3.5 * (1.0 + brightness_factor * 2.0)).clamp(200.0, 8000.0),
            filter_q: 1.2 + brightness_factor * 1.5,
            overall_gain: strike_factor * 0.9,
        };

        let duration = (decay_time * 1.5).max(0.5);
        let samples = 50usize;
        let envelope: Vec<(f64, f64)> = (0..samples)
            .map(|i| {
                let t = (i as f64) * duration / samples as f64;
                let env = if t < web_audio.attack_s {
                    t / web_audio.attack_s.max(1e-6)
                } else if t < web_audio.attack_s + web_audio.decay_s {
                    let td = t - web_audio.attack_s;
                    1.0 - (1.0 - web_audio.sustain_level) * td / web_audio.decay_s.max(1e-6)
                } else if t < duration - web_audio.release_s {
                    web_audio.sustain_level
                } else {
                    let tr = t - (duration - web_audio.release_s);
                    web_audio.sustain_level * (1.0 - tr / web_audio.release_s.max(1e-6)).max(0.0)
                };
                (t, env.max(0.0))
            })
            .collect();

        Some(VirtualTapResult {
            spectrum,
            fundamental_freq_hz: fund,
            harmonics,
            amplitude_envelope: envelope,
            decay_time_s: decay_time,
            brightness,
            timbre_character: timbre,
            zone_name,
            web_audio_params: web_audio,
        })
    }

    pub fn virtual_ensemble(&self, req: EnsembleTapRequest) -> EnsembleTapResult {
        let tempo = req.tempo_bpm.unwrap_or(90.0).clamp(40.0, 240.0);
        let beat_period_s = 60.0 / tempo;
        let pattern = req.rhythm_pattern.clone().unwrap_or_else(|| "unison".to_string());

        let n = req.taps.len().max(1);
        let offsets: Vec<f64> = match pattern.as_str() {
            "staggered" => (0..n).map(|i| i as f64 * beat_period_s / n as f64).collect(),
            "call_response" => (0..n).map(|i| if i % 2 == 0 { 0.0 } else { beat_period_s * 0.5 }).collect(),
            "polyrhythm" => (0..n).map(|i| {
                let factor = if i % 3 == 0 { 1.0 } else if i % 3 == 1 { 2.0 / 3.0 } else { 3.0 / 2.0 };
                (i as f64 * beat_period_s * 0.15 * factor) % beat_period_s
            }).collect(),
            _ => vec![0.0; n],
        };

        let individual: Vec<(Option<VirtualTapResult>, f64)> = req.taps.iter()
            .zip(offsets.iter())
            .map(|(t, off)| (self.virtual_tap(t.clone()), *off))
            .collect();

        let valid_results: Vec<&VirtualTapResult> = individual.iter()
            .filter_map(|(r, _)| r.as_ref()).collect();

        let mixed_spectrum = if valid_results.is_empty() {
            vec![]
        } else {
            let total_bins = valid_results.iter().map(|r| r.spectrum.len()).max().unwrap_or(0);
            let mut mixed = Vec::with_capacity(total_bins);
            for bi in 0..total_bins {
                let mut total_amp_pa = 0.0f64;
                let mut count = 0;
                for r in &valid_results {
                    if let Some(b) = r.spectrum.get(bi) {
                        let pa = 2.0e-5 * 10.0f64.powf(b.amplitude_db / 20.0);
                        total_amp_pa += pa;
                        count += 1;
                    }
                }
                if count > 0 {
                    let avg_pa = total_amp_pa / (count as f64).sqrt();
                    let db = 20.0 * (avg_pa.max(1e-12) / 2.0e-5).log10();
                    let avg_freq: f64 = valid_results.iter()
                        .filter_map(|r| r.spectrum.get(bi).map(|b| b.frequency_hz))
                        .sum::<f64>() / valid_results.len() as f64;
                    mixed.push(SpectrumBin { frequency_hz: avg_freq, amplitude_db: db });
                }
            }
            mixed
        };

        let total_duration = individual.iter()
            .filter_map(|(r, off)| r.as_ref().map(|rr| rr.decay_time_s * 1.5 + off))
            .fold(0.0f64, f64::max)
            .max(1.0);

        let samples = 60usize;
        let combined_envelope: Vec<(f64, f64)> = (0..samples)
            .map(|i| {
                let t = (i as f64) * total_duration / samples as f64;
                let mut amp = 0.0f64;
                for (r, off) in &individual {
                    if let Some(r) = r {
                        let local_t = t - off;
                        if local_t >= 0.0 {
                            if let Some((_, last)) = r.amplitude_envelope.last() {
                                amp += *last * (-local_t / r.decay_time_s.max(0.01)).exp().max(0.0);
                            }
                        }
                    }
                }
                (t, amp)
            })
            .collect();

        let synchronized = !offsets.is_empty() && offsets.iter().all(|o| *o < 0.01);

        let mut beat_intervals = Vec::new();
        if pattern == "polyrhythm" {
            for i in 1..offsets.len() {
                beat_intervals.push((offsets[i] - offsets[i - 1]).abs());
            }
        } else {
            for _ in 1..=(offsets.len().max(2)) {
                beat_intervals.push(beat_period_s);
            }
        }

        EnsembleTapResult {
            individual_results: individual.into_iter().filter_map(|(r, _)| r).collect(),
            mixed_spectrum,
            combined_envelope,
            synchronized,
            beat_intervals_s: beat_intervals,
            total_duration_s: total_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> VrDrumEnsemble {
        let config = Arc::new(crate::config::AppConfig::load());
        let pool = Arc::new(AcousticFemPool::new(config.clone()));
        let ethnic = Arc::new(EthnicComparator::new(config.clone(), pool.clone()));
        VrDrumEnsemble::new(config, pool, ethnic)
    }

    fn tap_req(drum_id: &str, zone: &str) -> VirtualTapRequest {
        let (x, y) = match zone {
            "center" => (0.5, 0.5),
            "edge" => (0.9, 0.5),
            _ => (0.7, 0.2),
        };
        VirtualTapRequest {
            drum_id: drum_id.to_string(),
            x_frac: x, y_frac: y,
            strike_force: 2.5,
            striker_type: Some("木槌".into()),
        }
    }

    #[test]
    fn test_center_vs_edge_zone_name_differs() {
        let c = make();
        let center = c.virtual_tap(tap_req("zhuang-dagu", "center")).unwrap();
        let edge = c.virtual_tap(tap_req("zhuang-dagu", "edge")).unwrap();
        assert_ne!(center.zone_name, edge.zone_name);
    }

    #[test]
    fn test_fundamental_within_audible_range() {
        let c = make();
        for did in ["zhuang-dagu", "miao-xiaogu", "yao-guzai"] {
            let r = c.virtual_tap(tap_req(did, "center")).unwrap();
            assert!((20.0..=10000.0).contains(&r.fundamental_freq_hz),
                "{did} fund={}", r.fundamental_freq_hz);
        }
    }

    #[test]
    fn test_decay_time_reasonable_range() {
        let c = make();
        let r = c.virtual_tap(tap_req("zhuang-dagu", "center")).unwrap();
        assert!((0.05..=30.0).contains(&r.decay_time_s), "decay={}", r.decay_time_s);
    }

    #[test]
    fn test_webaudio_adsr_params_valid() {
        let c = make();
        let r = c.virtual_tap(tap_req("zhuang-dagu", "center")).unwrap();
        let w = &r.web_audio_params;
        assert!(w.attack_s >= 0.0 && w.attack_s <= 1.0);
        assert!(w.decay_s >= 0.0 && w.decay_s <= 5.0);
        assert!((0.0..=1.0).contains(&w.sustain_level));
        assert!(w.release_s >= 0.0 && w.release_s <= 10.0);
        assert!(w.filter_cutoff_hz >= 20.0 && w.filter_cutoff_hz <= 20000.0);
        assert!(w.base_freq >= 20.0 && w.base_freq <= 10000.0);
    }

    #[test]
    fn test_invalid_drum_returns_none() {
        let c = make();
        assert!(c.virtual_tap(tap_req("nonexistent", "center")).is_none());
    }

    #[test]
    fn test_unison_ensemble_synchronized() {
        let c = make();
        let taps = vec![
            tap_req("zhuang-dagu", "center"),
            tap_req("miao-dagu", "center"),
        ];
        let req = EnsembleTapRequest {
            taps, tempo_bpm: Some(90.0), rhythm_pattern: Some("unison".into()),
        };
        let r = c.virtual_ensemble(req);
        assert!(r.synchronized, "unison should be synchronized");
        assert!(!r.mixed_spectrum.is_empty());
    }

    #[test]
    fn test_polyrhythm_ensemble_not_synchronized() {
        let c = make();
        let taps = vec![
            tap_req("zhuang-dagu", "center"),
            tap_req("miao-dagu", "center"),
            tap_req("yao-dagu", "center"),
        ];
        let req = EnsembleTapRequest {
            taps, tempo_bpm: Some(90.0), rhythm_pattern: Some("polyrhythm".into()),
        };
        let r = c.virtual_ensemble(req);
        assert!(!r.synchronized, "polyrhythm should not be synchronized");
    }
}
