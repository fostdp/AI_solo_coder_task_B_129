import { setupHiDPICanvas, drawMultiLineChart } from './chart_utils.js';

export class EraComparatorPanel {
  constructor({ apiGet, apiPost }) {
    this.apiGet = apiGet;
    this.apiPost = apiPost;
    this.ethnicLibrary = [];
    this.lastResult = null;
  }

  setup(ethnicLibrary) {
    this.ethnicLibrary = ethnicLibrary;
    this.populateSelects();
    const btn = document.getElementById('btn-crossera-compare');
    if (btn) btn.addEventListener('click', () => this.handleCompare());
  }

  populateSelects() {
    const ancientSelect = document.getElementById('crossera-ancient-select');
    if (ancientSelect && this.ethnicLibrary.length) {
      ancientSelect.innerHTML = this.ethnicLibrary.map(d =>
        `<option value="${d.drum_id}">${d.name} (${d.ethnic_group})</option>`
      ).join('');
    }
    const modernSelect = document.getElementById('crossera-modern-select');
    if (modernSelect) {
      [20, 23, 26, 29, 32].forEach(inch => {
        const opt = document.createElement('option');
        opt.value = inch;
        opt.textContent = `${inch}" 定音鼓`;
        modernSelect.appendChild(opt);
      });
      modernSelect.value = 29;
    }
  }

  async handleCompare() {
    const ancientId = document.getElementById('crossera-ancient-select').value;
    const timpaniSize = parseFloat(document.getElementById('crossera-modern-select').value);
    const drum = this.ethnicLibrary.find(d => d.drum_id === ancientId);
    if (drum) {
      try {
        const profile = await this.apiGet(`/api/experience/ethnic-drum/${ancientId}`);
        this.updateAncientInfo(profile);
      } catch (e) {
        this.updateAncientInfo({ ethnic_group: drum.ethnic_group, estimated_era: '古代', diameter_cm: drum.diameter_cm });
      }
    }
    const sizeText = { 20: 'C4–G4', 23: 'A3–E4', 26: 'F3–C4', 29: 'D3–A3', 32: 'Bb2–F3' };
    document.getElementById('crossera-modern-mat').textContent = '聚酯薄膜鼓皮 + 铜合金碗体';
    document.getElementById('crossera-modern-range').textContent = sizeText[timpaniSize] || '可调半音至六度';

    let result;
    try {
      result = await this.apiPost('/api/experience/cross-era', {
        ancient_drum_id: ancientId, timpani_size_inches: timpaniSize
      });
    } catch (e) {
      result = this.mockCompare(ancientId, timpaniSize);
    }
    this.lastResult = result;
    this.renderSpectrum(result);
    this.updateMetrics(result.metrics_comparison);
  }

  mockCompare(ancientId, timpaniSize) {
    const ancient = this.ethnicLibrary.find(d => d.drum_id === ancientId);
    const modernFund = 523.25 * Math.sqrt(29 / timpaniSize);
    const ancientFund = ancient ? ancient.fundamental_hz : 100;
    const genSpectrum = (base, decay) => {
      const bins = [];
      for (let i = 0; i < 15; i++) {
        bins.push({ frequency_hz: base * (i + 1), amplitude_db: 115 - i * decay });
      }
      return bins;
    };
    return {
      ancient_drum: { name: ancient ? ancient.name : ancientId },
      modern_drum: { name: `${timpaniSize}" 定音鼓` },
      ancient_spectrum: genSpectrum(ancientFund, 7),
      modern_spectrum: genSpectrum(modernFund, 3),
      metrics_comparison: {
        ancient_fundamental_hz: ancientFund,
        modern_fundamental_hz: modernFund,
        ancient_harmonic_ratios: [1.0, 1.59, 2.14, 2.65, 3.16, 3.6],
        modern_harmonic_ratios: [1.0, 1.505, 1.999, 2.492, 3.0, 3.5],
        ancient_decay_s: 2.5, modern_decay_s: 1.8,
        ancient_brightness: 0.4, modern_brightness: 0.7,
        ancient_radiated_power_w: 0.15, modern_radiated_power_w: 0.9,
        pitch_tunability: '古代铜鼓：铸造后不可调；现代定音鼓：踏板连续调半音~六度',
        cultural_context: '古代铜鼓：祭祀、仪式、权力象征；现代定音鼓：交响乐团、室内乐标准打击乐器',
      }
    };
  }

  updateAncientInfo(profile) {
    if (!profile) return;
    const el1 = document.getElementById('crossera-ancient-ethnic');
    if (el1 && profile.ethnic_group) el1.textContent = profile.ethnic_group;
    const el2 = document.getElementById('crossera-ancient-era');
    if (el2 && profile.estimated_era) el2.textContent = profile.estimated_era;
    const el3 = document.getElementById('crossera-ancient-diam');
    if (el3 && profile.diameter_cm) el3.textContent = `${profile.diameter_cm}cm`;
  }

  renderSpectrum(result) {
    const colors = ['#f59e0b', '#3b82f6'];
    const series = [
      { name: `古代: ${(result.ancient_drum || {}).name || '铜鼓'}`,
        data: (result.ancient_spectrum || []).slice(0, 20).map(b => [b.frequency_hz, Math.max(b.amplitude_db, 20)]) },
      { name: `现代: ${(result.modern_drum || {}).name || '定音鼓'}`,
        data: (result.modern_spectrum || []).slice(0, 20).map(b => [b.frequency_hz, Math.max(b.amplitude_db, 20)]) },
    ];
    const s = setupHiDPICanvas('crossera-spectrum', 480, 300);
    if (s) drawMultiLineChart(s.ctx, s.width, s.height, series, {
      title: '古代铜鼓 vs 现代定音鼓 频谱对比', xLabel: '频率 Hz', yLabel: '幅度 dB', yLog: true, colors
    });
  }

  updateMetrics(m) {
    if (!m) return;
    document.getElementById('crossera-ancient-freq').textContent = m.ancient_fundamental_hz.toFixed(1) + 'Hz';
    document.getElementById('crossera-modern-freq').textContent = m.modern_fundamental_hz.toFixed(1) + 'Hz';
    document.getElementById('crossera-ratio').textContent = (m.ancient_fundamental_hz / m.modern_fundamental_hz).toFixed(2);
    document.getElementById('crossera-cultural-note').textContent = m.cultural_context || '';
  }
}
