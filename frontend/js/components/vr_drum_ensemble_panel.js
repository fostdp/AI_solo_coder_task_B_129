import { setupHiDPICanvas, drawMultiLineChart } from './chart_utils.js';

export class VrDrumEnsemblePanel {
  constructor({ apiGet, apiPost }) {
    this.apiGet = apiGet;
    this.apiPost = apiPost;
    this.ethnicLibrary = [];
    this.currentDrumId = 'zhuang-dagu';
    this.audioCtx = null;
    this.lastTapTime = 0;
    this.lastResult = null;
    this.ensembleResult = null;
  }

  setup(ethnicLibrary) {
    this.ethnicLibrary = ethnicLibrary;
    this.populateDrumSelect();
    this.setupCanvasInteraction();
    this.setupEnsembleControls();
  }

  populateDrumSelect() {
    const sel = document.getElementById('play-drum-select');
    if (!sel || !this.ethnicLibrary.length) return;
    sel.innerHTML = this.ethnicLibrary.map(d =>
      `<option value="${d.drum_id}">${d.name}</option>`
    ).join('');
    sel.value = this.currentDrumId;
    sel.addEventListener('change', (e) => {
      this.currentDrumId = e.target.value;
      this.updateDrumInfo();
    });
    this.updateDrumInfo();
  }

  updateDrumInfo() {
    const d = this.ethnicLibrary.find(x => x.drum_id === this.currentDrumId);
    if (!d) return;
    const info = document.getElementById('play-drum-info');
    if (info) {
      info.innerHTML = `
        <div class="text-lg font-bold text-amber-400">${d.name}</div>
        <div class="text-sm text-slate-400 mt-1">${d.ethnic_group} · ${d.diameter_cm}cm · f₀=${d.fundamental_hz}Hz</div>
        <div class="text-xs text-slate-500 mt-1">📜 ${d.cultural_tag}</div>`;
    }
  }

  setupCanvasInteraction() {
    const canvas = document.getElementById('play-drum-canvas');
    if (!canvas) return;
    const handleTap = (e) => {
      const rect = canvas.getBoundingClientRect();
      const x = (e.clientX - rect.left) / rect.width;
      const y = (e.clientY - rect.top) / rect.height;
      this.performTap(x, y);
    };
    canvas.addEventListener('click', handleTap);
    canvas.addEventListener('touchstart', (e) => {
      e.preventDefault();
      const t = e.touches[0];
      const rect = canvas.getBoundingClientRect();
      const x = (t.clientX - rect.left) / rect.width;
      const y = (t.clientY - rect.top) / rect.height;
      this.performTap(x, y);
    });
    this.drawDrumFace(canvas, 0.5, 0.5);
  }

  drawDrumFace(canvas, xFrac, yFrac) {
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth, h = canvas.clientHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    canvas.style.width = w + 'px';
    canvas.style.height = h + 'px';
    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);

    const cx = w / 2, cy = h / 2;
    const radius = Math.min(w, h) / 2 - 10;

    const gradRim = ctx.createRadialGradient(cx, cy, radius * 0.9, cx, cy, radius);
    gradRim.addColorStop(0, '#b45309');
    gradRim.addColorStop(0.5, '#92400e');
    gradRim.addColorStop(1, '#78350f');
    ctx.fillStyle = gradRim;
    ctx.beginPath();
    ctx.arc(cx, cy, radius, 0, Math.PI * 2);
    ctx.fill();

    const gradFace = ctx.createRadialGradient(cx, cy, 0, cx, cy, radius * 0.9);
    gradFace.addColorStop(0, '#fbbf24');
    gradFace.addColorStop(0.6, '#f59e0b');
    gradFace.addColorStop(1, '#d97706');
    ctx.fillStyle = gradFace;
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.9, 0, Math.PI * 2);
    ctx.fill();

    ctx.strokeStyle = 'rgba(120,53,15,0.5)';
    ctx.lineWidth = 1.5;
    for (let i = 1; i <= 3; i++) {
      ctx.beginPath();
      ctx.arc(cx, cy, radius * 0.9 * i / 4, 0, Math.PI * 2);
      ctx.stroke();
    }

    const tx = cx + (xFrac - 0.5) * radius * 1.4;
    const ty = cy + (yFrac - 0.5) * radius * 1.4;
    const rFrac = Math.sqrt((xFrac - 0.5) ** 2 + (yFrac - 0.5) ** 2) * 2;
    const rippleMax = Math.min(radius * 0.3, radius * 0.9 - rFrac * radius);
    if (rippleMax > 0) {
      ctx.strokeStyle = 'rgba(255,255,255,0.5)';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(tx, ty, rippleMax * 0.3, 0, Math.PI * 2);
      ctx.stroke();
    }

    ctx.fillStyle = 'rgba(255,255,255,0.7)';
    ctx.beginPath();
    ctx.arc(tx, ty, 5, 0, Math.PI * 2);
    ctx.fill();
  }

  async performTap(xFrac, yFrac, force = 2.5) {
    const now = Date.now();
    if (now - this.lastTapTime < 50) return;
    this.lastTapTime = now;

    let result;
    try {
      result = await this.apiPost('/api/experience/virtual-tap', {
        drum_id: this.currentDrumId,
        x_frac: xFrac, y_frac: yFrac,
        strike_force: force, striker_type: '木槌'
      });
    } catch (e) {
      result = this.mockTap(xFrac, yFrac, force);
    }
    this.lastResult = result;
    this.playSound(result);
    this.renderTapResult(result);
    const canvas = document.getElementById('play-drum-canvas');
    if (canvas) this.drawDrumFace(canvas, xFrac, yFrac);
  }

  mockTap(xFrac, yFrac, force) {
    const d = this.ethnicLibrary.find(x => x.drum_id === this.currentDrumId);
    const fund = d ? d.fundamental_hz : 100;
    const rad = Math.sqrt((xFrac - 0.5) ** 2 + (yFrac - 0.5) ** 2) * 2;
    let zone, decay;
    if (rad < 0.25) { zone = '鼓心'; decay = 2.5; }
    else if (rad < 0.6) { zone = '鼓中'; decay = 1.8; }
    else { zone = '鼓边'; decay = 1.0; }

    const spectrum = [];
    for (let i = 0; i < 12; i++) {
      spectrum.push({ frequency_hz: fund * (i + 1), amplitude_db: 105 - i * 5 });
    }
    const harmonics = [];
    for (let i = 0; i < 8; i++) harmonics.push([fund * (i + 1), 1 / (i + 1)]);

    const envelope = [];
    for (let i = 0; i < 30; i++) {
      const t = i * 0.1;
      const amp = Math.exp(-t / decay) * force;
      envelope.push([t, amp]);
    }
    return {
      spectrum,
      fundamental_freq_hz: fund,
      harmonics,
      amplitude_envelope: envelope,
      decay_time_s: decay,
      brightness: rad < 0.25 ? 0.4 : rad < 0.6 ? 0.6 : 0.85,
      timbre_character: zone + '音色',
      zone_name: zone,
      web_audio_params: {
        base_freq: fund, harmonic_gains: [1, 0.7, 0.5, 0.35, 0.25, 0.18],
        attack_s: 0.005, decay_s: 0.15, sustain_level: 0.35, release_s: 0.8,
        filter_cutoff_hz: fund * 5, filter_q: 1.5, overall_gain: force / 2.5,
      }
    };
  }

  playSound(result) {
    if (!result?.web_audio_params) return;
    if (!this.audioCtx) {
      try {
        this.audioCtx = new (window.AudioContext || window.webkitAudioContext)();
      } catch (e) { return; }
    }
    const ctx = this.audioCtx;
    if (ctx.state === 'suspended') ctx.resume();
    const p = result.web_audio_params;
    const now = ctx.currentTime;
    const oscCount = (p.harmonic_gains || [1]).length;
    const masterGain = ctx.createGain();
    masterGain.gain.value = 0.3 * (p.overall_gain || 1);
    const filter = ctx.createBiquadFilter();
    filter.type = 'lowpass';
    filter.frequency.value = p.filter_cutoff_hz || 2000;
    filter.Q.value = p.filter_q || 1;
    masterGain.connect(filter);
    filter.connect(ctx.destination);

    for (let i = 0; i < oscCount; i++) {
      const osc = ctx.createOscillator();
      osc.type = 'sine';
      osc.frequency.value = p.base_freq * (i + 1);
      const gain = ctx.createGain();
      gain.gain.value = 0;
      osc.connect(gain);
      gain.connect(masterGain);
      const amp = (p.harmonic_gains[i] || 0.5) / (i + 1);
      gain.gain.setValueAtTime(0, now);
      gain.gain.linearRampToValueAtTime(amp, now + (p.attack_s || 0.01));
      gain.gain.exponentialRampToValueAtTime(
        amp * (p.sustain_level || 0.3),
        now + (p.attack_s || 0.01) + (p.decay_s || 0.1)
      );
      osc.start(now);
      gain.gain.exponentialRampToValueAtTime(
        0.0001, now + (p.attack_s || 0.01) + (p.decay_s || 0.1) + (p.release_s || 0.5)
      );
      osc.stop(now + (p.attack_s || 0.01) + (p.decay_s || 0.1) + (p.release_s || 0.5) + 0.1);
    }
  }

  renderTapResult(result) {
    const zoneEl = document.getElementById('play-zone-info');
    if (zoneEl) zoneEl.textContent = `敲击区域：${result.zone_name} (${result.timbre_character})`;
    const fundEl = document.getElementById('play-fundamental');
    if (fundEl) fundEl.textContent = result.fundamental_freq_hz.toFixed(1) + ' Hz';
    const decayEl = document.getElementById('play-decay');
    if (decayEl) decayEl.textContent = result.decay_time_s.toFixed(2) + ' s';

    const series = [{ name: '频谱', data: (result.spectrum || []).slice(0, 15).map(b => [b.frequency_hz, Math.max(b.amplitude_db, 20)]) }];
    const s = setupHiDPICanvas('play-spectrum-canvas', 380, 180);
    if (s) drawMultiLineChart(s.ctx, s.width, s.height, series, {
      title: '实时频谱', xLabel: 'Hz', yLabel: 'dB', yLog: true, colors: ['#10b981']
    });
  }

  setupEnsembleControls() {
    const btn = document.getElementById('play-ensemble-btn');
    if (!btn) return;
    btn.addEventListener('click', () => this.performEnsemble());
  }

  async performEnsemble() {
    const pattern = document.getElementById('play-ensemble-pattern')?.value || 'unison';
    const tempo = parseFloat(document.getElementById('play-ensemble-tempo')?.value || '90');
    const drumSelect = document.getElementById('play-ensemble-drums');
    const drumIds = [];
    if (drumSelect) {
      for (const opt of drumSelect.selectedOptions) drumIds.push(opt.value);
    }
    if (!drumIds.length) drumIds.push(this.currentDrumId);

    const taps = drumIds.map(id => ({
      drum_id: id, x_frac: 0.5, y_frac: 0.5,
      strike_force: 2.5, striker_type: '木槌'
    }));

    let result;
    try {
      result = await this.apiPost('/api/experience/virtual-ensemble', {
        taps, tempo_bpm: tempo, rhythm_pattern: pattern,
      });
    } catch (e) {
      result = this.mockEnsemble(taps, tempo, pattern);
    }
    this.ensembleResult = result;
    this.renderEnsembleResult(result);
  }

  mockEnsemble(taps, tempo, pattern) {
    const period = 60 / tempo;
    const n = taps.length;
    let offsets = [];
    switch (pattern) {
      case 'staggered': offsets = taps.map((_, i) => i * period / n); break;
      case 'call_response': offsets = taps.map((_, i) => i % 2 === 0 ? 0 : period / 2); break;
      case 'polyrhythm': offsets = taps.map((_, i) => (i * 0.15 * period * (i % 3 === 0 ? 1 : i % 3 === 1 ? 2 / 3 : 1.5)) % period); break;
      default: offsets = taps.map(() => 0);
    }
    const individuals = taps.map((t, i) => ({ ...this.mockTap(0.5, 0.5, 2.5), offset_s: offsets[i] }));
    const mixLen = 20;
    const mixed = [];
    for (let i = 0; i < mixLen; i++) {
      const f = 100 * (i + 1);
      mixed.push({ frequency_hz: f, amplitude_db: 100 - i * 4 });
    }
    const totalDur = period * 2;
    const env = [];
    const samp = 30;
    for (let i = 0; i < samp; i++) {
      const t = i * totalDur / samp;
      let a = 0;
      for (let j = 0; j < n; j++) {
        const localT = t - offsets[j];
        if (localT >= 0) a += Math.exp(-localT / 1.5);
      }
      env.push([t, a]);
    }
    const sync = offsets.every(o => o < 0.01);
    return {
      individual_results: individuals,
      mixed_spectrum: mixed,
      combined_envelope: env,
      synchronized: sync,
      beat_intervals_s: sync ? [period] : offsets.slice(1).map((o, i) => Math.abs(o - offsets[i])),
      total_duration_s: totalDur,
    };
  }

  renderEnsembleResult(result) {
    const syncEl = document.getElementById('play-ensemble-sync');
    if (syncEl) syncEl.textContent = result.synchronized ? '✅ 节拍同步' : '⚠️ 多节奏异步';
    const durEl = document.getElementById('play-ensemble-duration');
    if (durEl) durEl.textContent = result.total_duration_s.toFixed(2) + ' s';
    const series = [{ name: '合奏包络', data: result.combined_envelope || [] }];
    const s = setupHiDPICanvas('play-ensemble-canvas', 380, 160);
    if (s) drawMultiLineChart(s.ctx, s.width, s.height, series, {
      title: '合奏波形包络', xLabel: '时间 s', yLabel: '幅度', colors: ['#8b5cf6']
    });
  }
}
