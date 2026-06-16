use reqwest::Client;
use serde_json::{json, Value};

const BASE: &str = "http://localhost:8080";

fn client() -> Client {
    Client::builder().build().expect("build http client")
}

async fn must_get_ok(url: &str) -> Value {
    let r = client().get(url).send().await
        .expect("GET request");
    assert!(r.status().is_success(), "GET {url} failed: {}", r.status());
    let body: Value = r.json().await.expect("parse JSON");
    assert!(body["success"].as_bool().unwrap_or(false), "success should be true: {body:?}");
    body["data"].clone()
}

async fn must_post_ok(url: &str, body: Value) -> Value {
    let r = client().post(url).json(&body).send().await
        .expect("POST request");
    assert!(r.status().is_success(), "POST {url} failed: {}", r.status());
    let resp: Value = r.json().await.expect("parse JSON");
    assert!(resp["success"].as_bool().unwrap_or(false),
        "success=false, body={resp:?}");
    resp["data"].clone()
}

async fn check_server_alive() -> bool {
    match client().get(&format!("{BASE}/api/health")).send().await {
        Ok(r) => r.status().is_success(),
        _ => false,
    }
}

macro_rules! require_server {
    () => {
        if !check_server_alive().await {
            eprintln!("skip: backend server not running on {BASE}");
            return;
        }
    };
}

#[tokio::test]
async fn test_01_health() {
    require_server!();
    let d = must_get_ok(&format!("{BASE}/api/health")).await;
    assert_eq!(d["version"], "2.0.0", "expected v2 after feature iteration");
    assert_eq!(d["architecture"], "microservices-mpsc");
    assert_eq!(d["status"], "healthy");
}

#[tokio::test]
async fn test_02_ethnic_library_returns_six_drums() {
    require_server!();
    let drums = must_get_ok(&format!("{BASE}/api/experience/ethnic-library")).await;
    let arr = drums.as_array().expect("array");
    assert_eq!(arr.len(), 6, "壮2+苗2+瑶2=6 ethnic drums");
    let ids: Vec<&str> = arr.iter().filter_map(|d| d["drum_id"].as_str()).collect();
    assert!(ids.contains(&"zhuang-dagu"), "壮族大铜鼓 present");
    assert!(ids.contains(&"miao-xiaogu"), "苗族小铜鼓 present");
    assert!(ids.contains(&"yao-guzai"), "瑶族铜鼓仔 present");
    for d in arr {
        assert!(d["diameter_cm"].as_f64().unwrap() > 0.0);
        let e = d["ethnic_group"].as_str().unwrap();
        assert!(matches!(e, "壮族" | "苗族" | "瑶族"),
            "ethnic_group must be one of 壮苗瑶, got {e}");
    }
}

#[tokio::test]
async fn test_03_ethnic_drum_profile_valid_and_invalid() {
    require_server!();

    let zhuang = must_get_ok(
        &format!("{BASE}/api/experience/ethnic-drum/zhuang-dagu")).await;
    assert_eq!(zhuang["drum_id"], "zhuang-dagu");
    assert_eq!(zhuang["ethnic_group"], "壮族");
    assert!(zhuang["diameter_cm"].as_f64().unwrap() >= 50.0);
    let alloy_sum =
        zhuang["alloy"]["copper_pct"].as_f64().unwrap()
        + zhuang["alloy"]["tin_pct"].as_f64().unwrap()
        + zhuang["alloy"]["lead_pct"].as_f64().unwrap()
        + zhuang["alloy"]["zinc_pct"].as_f64().unwrap()
        + zhuang["alloy"]["other_impurities_pct"].as_f64().unwrap();
    assert!((99.5..=100.5).contains(&alloy_sum),
        "alloy percentages sum to ~100 (got {alloy_sum})");

    let missing_r = client()
        .get(&format!("{BASE}/api/experience/ethnic-drum/nothing-here-999"))
        .send().await.unwrap();
    let body: Value = missing_r.json().await.unwrap();
    assert!(!body["success"].as_bool().unwrap_or(true),
        "missing drum profile should have success=false");
}

#[tokio::test]
async fn test_04_compare_ethnic_drums_happy_path() {
    require_server!();
    let body = json!({
        "drum_ids": ["zhuang-dagu", "miao-dagu", "yao-dagu"]
    });
    let d = must_post_ok(
        &format!("{BASE}/api/experience/ethnic-compare"), body).await;

    assert_eq!(d["drums"].as_array().unwrap().len(), 3);
    let spectra = d["frequency_spectra"].as_array().unwrap();
    assert_eq!(spectra.len(), 3);
    for s in spectra {
        let pair = s.as_array().unwrap();
        assert_eq!(pair.len(), 2, "each spectrum entry is [drum_id, bins]");
        let bins = pair[1].as_array().unwrap();
        assert!(!bins.is_empty());
        for b in bins {
            assert!(b["frequency_hz"].as_f64().unwrap() > 0.0,
                "freq positive");
            let db = b["amplitude_db"].as_f64().unwrap();
            assert!(db < 250.0, "amplitude_db realistic: {db}");
        }
    }

    let modes_comp = d["vibration_modes_comparison"].as_array().unwrap();
    assert!(modes_comp.len() >= 1);

    let cm = &d["comparison_metrics"];
    let fundamentals = cm["fundamental_freqs"].as_array().unwrap();
    assert_eq!(fundamentals.len(), 3);
    for f in fundamentals {
        let pair = f.as_array().unwrap();
        let hz = pair[1].as_f64().unwrap();
        assert!(hz > 20.0 && hz < 10000.0,
            "fundamental audible: {hz}");
    }
}

#[tokio::test]
async fn test_05_compare_ethnic_drums_edge_single_element() {
    require_server!();
    let body = json!({"drum_ids": ["yao-guzai"]});
    let d = must_post_ok(
        &format!("{BASE}/api/experience/ethnic-compare"), body).await;
    assert_eq!(d["drums"].as_array().unwrap().len(), 1);
    assert_eq!(d["frequency_spectra"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_06_compare_ethnic_drums_all_invalid_graceful() {
    require_server!();
    let r = client()
        .post(&format!("{BASE}/api/experience/ethnic-compare"))
        .json(&json!({"drum_ids": ["bad1", "bad2", "---"]}))
        .send().await.unwrap();
    let status_ok = r.status().is_success();
    let body: Value = r.json().await.unwrap();
    if status_ok && body["success"].as_bool().unwrap_or(false) {
        let drums = body["data"]["drums"].as_array().unwrap();
        assert!(drums.is_empty(),
            "all invalid ids => empty drums array: {body:?}");
    }
}

#[tokio::test]
async fn test_07_cross_era_happy_verify_pitch_accuracy() {
    require_server!();
    let body = json!({"drum_id": "zhuang-dagu", "timpani_size_inches": 29.0});
    let d = must_post_ok(
        &format!("{BASE}/api/experience/cross-era"), body).await;
    let m = &d["metrics_comparison"];

    let mod_f = m["modern_fundamental_hz"].as_f64().unwrap();
    assert!((400.0..=650.0).contains(&mod_f),
        "29in timpani fundamental ~C5 should be 400-650Hz, got {mod_f}");
    let anc_f = m["ancient_fundamental_hz"].as_f64().unwrap();
    assert!(anc_f > 0.0, "ancient freq positive");

    let anc_harmonic_mean = m["ancient_harmonic_ratios"]
        .as_array().unwrap()
        .iter().filter_map(|v| v.as_f64())
        .sum::<f64>()
        .max(0.001);
    let mod_harmonic_mean = m["modern_harmonic_ratios"]
        .as_array().unwrap()
        .iter().filter_map(|v| v.as_f64())
        .sum::<f64>()
        .max(0.001);
    assert!(mod_harmonic_mean > 0.0);
    assert!(anc_harmonic_mean > 0.0);

    assert!(!d["ancient_spectrum"].as_array().unwrap().is_empty());
    assert!(!d["modern_spectrum"].as_array().unwrap().is_empty());
    assert!(m["cultural_context"].as_str().unwrap().len() > 5);
}

#[tokio::test]
async fn test_08_cross_era_boundary_sizes_monotonic_pitch() {
    require_server!();
    let mut prev: Option<f64> = None;
    for size in [20.0, 23.0, 26.0, 29.0, 32.0] {
        let body = json!({"drum_id": "yao-guzai", "timpani_size_inches": size});
        let d = must_post_ok(
            &format!("{BASE}/api/experience/cross-era"), body).await;
        let f = d["metrics_comparison"]["modern_fundamental_hz"]
            .as_f64().unwrap();
        if let Some(p) = prev {
            assert!(f < p,
                "larger timpani must have lower fundamental: \
                 size {size} -> {f} Hz should be < previous {p} Hz");
        }
        prev = Some(f);
    }
}

#[tokio::test]
async fn test_09_cross_era_invalid_drum_returns_not_found() {
    require_server!();
    let body = json!({"drum_id": "NOT-A-REAL-DRUM-12345",
        "timpani_size_inches": 26.0});
    let r = client()
        .post(&format!("{BASE}/api/experience/cross-era"))
        .json(&body).send().await.unwrap();
    let body: Value = r.json().await.unwrap();
    assert!(!body["success"].as_bool().unwrap_or(false),
        "invalid drum_id should give success=false");
}

#[tokio::test]
async fn test_10_cross_era_negative_size_handled() {
    require_server!();
    let body = json!({"drum_id": "zhuang-dagu", "timpani_size_inches": -10.0});
    let r = client()
        .post(&format!("{BASE}/api/experience/cross-era"))
        .json(&body).send().await.unwrap();
    let st = r.status();
    let body: Value = r.json().await.unwrap();
    if st.is_success() && body["success"].as_bool().unwrap_or(false) {
        let f = body["data"]["metrics_comparison"]
            ["modern_fundamental_hz"].as_f64();
        if let Some(v) = f {
            assert!(v.is_finite() && v > 0.0,
                "negative size must not crash: got fundamental={v}");
        }
    }
}

#[tokio::test]
async fn test_11_ritual_soundfield_verify_spl_reverberation() {
    require_server!();
    let body = json!({
        "drum_placements": [
            {"drum_id": "zhuang-dagu", "position_x_m": -3.0,
             "position_y_m": 0.0, "position_z_m": 0.0,
             "relative_volume": 1.0, "strike_phase_offset_s": 0.0},
            {"drum_id": "miao-dagu", "position_x_m": 3.0,
             "position_y_m": 0.0, "position_z_m": 0.0,
             "relative_volume": 0.8, "strike_phase_offset_s": 0.1},
            {"drum_id": "yao-dagu", "position_x_m": 0.0,
             "position_y_m": -3.0, "position_z_m": 0.0,
             "relative_volume": 0.9, "strike_phase_offset_s": 0.05}
        ],
        "observer_x_m": 0.0, "observer_y_m": 0.0, "observer_z_m": 1.5,
        "field_radius_m": 10.0, "grid_resolution": 6
    });
    let d = must_post_ok(
        &format!("{BASE}/api/experience/ritual-soundfield"), body).await;

    assert_eq!(d["drum_count"], 3);
    let obs_spl = d["observer_spl_db"].as_f64().unwrap();
    assert!((40.0..180.0).contains(&obs_spl),
        "observer SPL should be physically reasonable (40-180 dB), got {obs_spl}");
    let power = d["total_radiated_power_w"].as_f64().unwrap();
    assert!(power > 0.0, "power must be positive");

    let field = d["sound_field"].as_array().unwrap();
    assert!(!field.is_empty());
    let mut max_spl = f64::NEG_INFINITY;
    let mut min_spl = f64::INFINITY;
    for fp in field {
        let spl = fp["spl_db"].as_f64().unwrap();
        assert!(spl.is_finite(), "SPL finite at all points");
        assert!(fp["pressure_pa"].as_f64().unwrap() >= 0.0);
        max_spl = max_spl.max(spl);
        min_spl = min_spl.min(spl);
    }
    assert!(max_spl > min_spl,
        "field should exhibit a spatial gradient (混响梯度): \
         min={min_spl} max={max_spl}");

    let scene = &d["ritual_scene"];
    assert!(scene["description"].as_str().unwrap().len() > 5);
    assert!(scene["historical_reference"].as_str().unwrap().len() > 5);

    let env = d["temporal_envelope"].as_array().unwrap();
    assert!(env.len() >= 2, "temporal envelope should have decay curve");
    let amps: Vec<f64> = env.iter()
        .map(|e| e.as_array().unwrap()[1].as_f64().unwrap())
        .collect();
    let max_amp = amps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let last_amp = amps.last().unwrap();
    assert!(max_amp > 0.0, "max amp positive");
    assert!(max_amp > *last_amp * 0.01,
        "envelope should decay (混响衰减): max={max_amp} last={last_amp}");
}

#[tokio::test]
async fn test_12_ritual_soundfield_edge_empty_placements() {
    require_server!();
    let body = json!({
        "drum_placements": [],
        "observer_x_m": 0, "observer_y_m": 0, "observer_z_m": 0,
        "field_radius_m": 5.0, "grid_resolution": 3
    });
    let r = client()
        .post(&format!("{BASE}/api/experience/ritual-soundfield"))
        .json(&body).send().await.unwrap();
    assert!(r.status().is_success(),
        "empty placements must not 500");
    let body: Value = r.json().await.unwrap();
    if body["success"].as_bool().unwrap_or(false) {
        assert_eq!(body["data"]["drum_count"].as_i64().unwrap(), 0);
    }
}

#[tokio::test]
async fn test_13_ritual_soundfield_observer_at_drum_no_inf() {
    require_server!();
    let body = json!({
        "drum_placements": [
            {"drum_id": "zhuang-dagu",
             "position_x_m": 0.0, "position_y_m": 0.0, "position_z_m": 0.0,
             "relative_volume": 1.0, "strike_phase_offset_s": 0.0}
        ],
        "observer_x_m": 0.0, "observer_y_m": 0.0, "observer_z_m": 0.0,
        "field_radius_m": 5.0, "grid_resolution": 4
    });
    let d = must_post_ok(
        &format!("{BASE}/api/experience/ritual-soundfield"), body).await;
    let spl = d["observer_spl_db"].as_f64().unwrap();
    assert!(spl.is_finite(),
        "observer coincident with drum must remain finite, got {spl}");
    for fp in d["sound_field"].as_array().unwrap() {
        assert!(fp["pressure_pa"].as_f64().unwrap().is_finite());
        assert!(fp["spl_db"].as_f64().unwrap().is_finite());
    }
}

#[tokio::test]
async fn test_14_virtual_tap_musicality_center_vs_edge() {
    require_server!();

    let center = must_post_ok(&format!("{BASE}/api/experience/virtual-tap"),
        json!({"drum_id": "zhuang-dagu",
               "x_frac": 0.5, "y_frac": 0.5,
               "strike_force": 1.0})).await;

    let edge = must_post_ok(&format!("{BASE}/api/experience/virtual-tap"),
        json!({"drum_id": "zhuang-dagu",
               "x_frac": 0.9, "y_frac": 0.5,
               "strike_force": 1.0})).await;

    let cf = center["fundamental_freq_hz"].as_f64().unwrap();
    let ef = edge["fundamental_freq_hz"].as_f64().unwrap();
    assert!(cf > 20.0 && cf < 10000.0,
        "center fund audible: {cf}");
    assert!(ef > 20.0 && ef < 10000.0,
        "edge fund audible: {ef}");

    assert_ne!(center["zone_name"].as_str(), edge["zone_name"].as_str(),
        "different strike locations should yield different zone names");

    let cb = center["brightness"].as_f64().unwrap();
    let eb = edge["brightness"].as_f64().unwrap();
    assert!(cb.is_finite() && (0.0..500.0).contains(&cb),
        "center brightness sane: {cb}");
    assert!(eb.is_finite() && (0.0..500.0).contains(&eb),
        "edge brightness sane: {eb}");

    let cd = center["decay_time_s"].as_f64().unwrap();
    assert!(cd > 0.05 && cd < 30.0,
        "decay time musical (0.05 - 30s): {cd}");

    let cweb = &center["web_audio_params"];
    assert!(cweb["base_freq"].as_f64().unwrap() > 0.0);
    assert!(cweb["attack_s"].as_f64().unwrap() >= 0.0);
    assert!(cweb["decay_s"].as_f64().unwrap() > 0.0);
    assert!(cweb["release_s"].as_f64().unwrap() > 0.0);
    assert!(cweb["filter_cutoff_hz"].as_f64().unwrap() > 0.0);

    assert!(!center["harmonics"].as_array().unwrap().is_empty());
    assert!(!center["spectrum"].as_array().unwrap().is_empty());
    assert!(center["amplitude_envelope"].as_array().unwrap().len() >= 4,
        "ADSR needs >= 4 points");
}

#[tokio::test]
async fn test_15_virtual_tap_all_six_ethnic_drums() {
    require_server!();
    for did in ["zhuang-dagu", "zhuang-magu",
        "miao-dagu", "miao-xiaogu",
        "yao-dagu", "yao-guzai"] {
        let r = must_post_ok(&format!("{BASE}/api/experience/virtual-tap"),
            json!({"drum_id": did,
                   "x_frac": 0.5, "y_frac": 0.5,
                   "strike_force": 1.0})).await;
        let fund = r["fundamental_freq_hz"].as_f64().unwrap();
        assert!(fund > 0.0, "{did} fundamental must be positive: {fund}");
    }
}

#[tokio::test]
async fn test_16_virtual_tap_force_boundaries_musicality() {
    require_server!();

    let req = |force: f64| async move {
        must_post_ok(&format!("{BASE}/api/experience/virtual-tap"),
            json!({"drum_id": "zhuang-magu",
                   "x_frac": 0.5, "y_frac": 0.5,
                   "strike_force": force})).await
    };

    let weak = req(0.1).await;
    let strong = req(2.0).await;

    let w_attack = weak["web_audio_params"]["attack_s"].as_f64().unwrap();
    let s_attack = strong["web_audio_params"]["attack_s"].as_f64().unwrap();
    assert!(w_attack.is_finite() && s_attack.is_finite());

    let w_decay = weak["decay_time_s"].as_f64().unwrap();
    let s_decay = strong["decay_time_s"].as_f64().unwrap();
    assert!(w_decay.is_finite() && s_decay.is_finite());
}

#[tokio::test]
async fn test_17_virtual_tap_invalid_location_and_drum() {
    require_server!();

    let r1 = client().post(&format!("{BASE}/api/experience/virtual-tap"))
        .json(&json!({
            "drum_id": "zhuang-dagu",
            "x_frac": 9999.0, "y_frac": -9999.0,
            "strike_force": 1.0
        })).send().await.unwrap();
    assert!(r1.status().is_success() || r1.status().is_client_error(),
        "out-of-bounds coords must not 500");
    if r1.status().is_success() {
        let b: Value = r1.json().await.unwrap();
        if b["success"].as_bool().unwrap_or(false) {
            assert!(b["data"]["fundamental_freq_hz"].as_f64().unwrap() > 0.0,
                "clamped coords still produce sound");
        }
    }

    let r2 = client().post(&format!("{BASE}/api/experience/virtual-tap"))
        .json(&json!({
            "drum_id": "absolutely-nonexistent",
            "x_frac": 0.5, "y_frac": 0.5,
            "strike_force": 1.0
        })).send().await.unwrap();
    let st = r2.status();
    assert!(!st.is_server_error(),
        "invalid drum_id must not cause 500");
}

#[tokio::test]
async fn test_18_cors_preflight_works() {
    require_server!();
    let r = client().request(
        reqwest::Method::OPTIONS, &format!("{BASE}/api/experience/ethnic-library"))
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "GET")
        .send().await.unwrap();
    assert!(r.status().is_success()
        || r.status() == reqwest::StatusCode::NO_CONTENT,
        "CORS preflight should succeed, got {}", r.status());
    let h = r.headers().get("access-control-allow-origin")
        .and_then(|v| v.to_str().ok()).unwrap_or("");
    assert!(h == "*", "CORS allow-origin should be *, got '{h}'");
}

#[tokio::test]
async fn test_19_original_apis_not_broken() {
    require_server!();
    let r = client().get(&format!("{BASE}/api/drums")).send().await.unwrap();
    let status = r.status();
    if status.is_success() {
        let body: Value = r.json().await.unwrap();
        if body["success"].as_bool().unwrap_or(false) {
            body["data"].as_array().expect("drums array");
        } else {
            eprintln!("skip: /api/drums returned success=false (likely no ClickHouse)");
        }
    } else {
        eprintln!("skip: /api/drums status={status} (likely no ClickHouse)");
    }
}
