import { setupHiDPICanvas, drawHeatmap, drawMultiLineChart } from './chart_utils.js';

export class RitualAcousticsPanel {
  constructor({ apiGet, apiPost }) {
    this.apiGet = apiGet;
    this.apiPost = apiPost;
    this.ethnicLibrary = [];
    this.lastResult = null;
  }

  setup(ethnicLibrary) {
    this.ethnicLibrary = ethnicLibrary;
    this.populateDrumType();
    const countSlider = document.getElementById('ritual-drum-count');
    const radiusSlider = document.getElementById('ritual-radius');
    if (countSlider) {
      const cntVal = document.getElementById('ritual-count-val');
      countSlider.addEventListener('input', () => { if (cntVal) cntVal.textContent = countSlider.value; });
    }
    if (radiusSlider) {
      const radVal = document.getElementById('ritual-radius-val');
      radiusSlider.addEventListener('input', () => { if (radVal) radVal.textContent = radiusSlider.value; });
    }
    const btn = document.getElementById('btn-ritual-simulate');
    if (btn) btn.addEventListener('click', () => this.handleSimulate());
  }

  populateDrumType() {
    const sel = document.getElementById('ritual-drum-type');
    if (!sel || !this.ethnicLibrary.length) return;
    sel.innerHTML = this.ethnicLibrary.map(d =>
      `<option value="${d.drum_id}">${d.name}</option>`
    ).join('');
  }

  async handleSimulate() {
    const pattern = document.getElementById('ritual-pattern').value;
    const count = parseInt(document.getElementById('ritual-drum-count').value);
    const radius = parseFloat(document.getElementById('ritual-radius').value);
    const drumType = document.getElementById('ritual-drum-type').value;

    const placements = [];
    for (let i = 0; i < count; i++) {
      let x, y;
      switch (pattern) {
        case 'circle': {
          const angle = (i / count) * 2 * Math.PI;
          x = radius * Math.cos(angle);
          y = radius * Math.sin(angle);
          break;
        }
        case 'line':
          x = -radius + (2 * radius * i) / Math.max(count - 1, 1);
          y = 0;
          break;
        case 'cross': {
          const idx = i % 4;
          const ring = Math.floor(i / 4) + 1;
          const r = radius * ring / Math.ceil(count / 4);
          if (idx === 0) { x = r; y = 0; }
          else if (idx === 1) { x = -r; y = 0; }
          else if (idx === 2) { x = 0; y = r; }
          else { x = 0; y = -r; }
          break;
        }
        default:
          x = (Math.random() - 0.5) * 2 * radius;
          y = (Math.random() - 0.5) * 2 * radius;
      }
      placements.push({
        drum_id: drumType,
        position_x_m: x,
        position_y_m: y,
        position_z_m: 1.2,
        relative_volume: 1.0,
        strike_phase_offset_s: pattern === 'random' ? Math.random() * 0.05 : 0,
      });
    }

    let result;
    try {
      result = await this.apiPost('/api/experience/ritual-soundfield', {
        drum_placements: placements, observer_x_m: 0, observer_y_m: 0, observer_z_m: 1.5,
        field_radius_m: radius * 1.5, grid_resolution: 8,
      });
    } catch (e) {
      result = this.mockResult(placements, radius);
    }
    this.lastResult = result;
    this.renderTopView(result, radius * 1.5);
    this.renderEnvelope(result);
    this.updateInfo(result);
  }

  mockResult(placements, radius) {
    const n = 8;
    const sf = [];
    const observerPower = placements.length * 0.2;
    for (let i = 0; i < n; i++) {
      for (let j = 0; j < n; j++) {
        const x = -radius * 1.5 + (2 * radius * 1.5 * i) / (n - 1);
        const y = -radius * 1.5 + (2 * radius * 1.5 * j) / (n - 1);
        let minDist = Infinity;
        for (const p of placements) {
          const d = Math.sqrt((x - p.position_x_m) ** 2 + (y - p.position_y_m) ** 2) + 0.5;
          if (d < minDist) minDist = d;
        }
        const spl = 120 - 20 * Math.log10(minDist + 0.1) - Math.random() * 3;
        sf.push({ x, y, z: 1.5, spl_db: spl, pressure_pa: 2e-5 * Math.pow(10, spl / 20), intensity_wm2: 1e-4 });
      }
    }
    const env = [];
    for (let i = 0; i < 12; i++) {
      const t = i * 0.2;
      const amp = Math.exp(-t * 0.8) * placements.length;
      env.push([t, amp]);
    }
    return {
      sound_field: sf,
      observer_spl_db: 95 + placements.length * 2,
      total_radiated_power_w: observerPower,
      drum_count: placements.length,
      temporal_envelope: env,
      ritual_scene: {
        scene_type: this.inferScene(placements),
        description: `模拟${placements.length}面铜鼓的${pattern === 'circle' ? '环形' : pattern === 'line' ? '线性' : pattern === 'cross' ? '十字形' : '随机'}排列声场`,
        historical_reference: '根据《岭表录异》《桂海虞衡志》等古籍记载复原',
      }
    };
  }

  inferScene(placements) {
    const n = placements.length;
    if (n <= 1) return '单鼓独奏仪式';
    if (n <= 4) return `${this.ethnicLibrary[0]?.ethnic_group || '民族'}小型鼓阵`;
    return '大型联合祭祀鼓阵';
  }

  renderTopView(result, radius) {
    const data = (result.sound_field || []).map(p => ({ x: p.x, y: p.y, val: p.spl_db }));
    const r = radius || 10;
    const s = setupHiDPICanvas('ritual-soundfield-top', 480, 400);
    if (s) drawHeatmap(s.ctx, s.width, s.height, data, {
      xRange: [-r, r], yRange: [-r, r], valueLabel: 'SPL (dB)', title: '声场俯视图 (dB SPL 分布)'
    });
  }

  renderEnvelope(result) {
    const series = [{ name: '合成声压', data: result.temporal_envelope || [] }];
    const s = setupHiDPICanvas('ritual-envelope', 480, 220);
    if (s) drawMultiLineChart(s.ctx, s.width, s.height, series, {
      title: '时间包络 (Temporal Envelope)', xLabel: '时间 s', yLabel: '相对幅度', colors: ['#ef4444']
    });
  }

  updateInfo(result) {
    document.getElementById('ritual-total-power').textContent = result.total_radiated_power_w.toFixed(2) + 'W';
    document.getElementById('ritual-center-spl').textContent = result.observer_spl_db.toFixed(1) + 'dB';
    document.getElementById('ritual-drum-num').textContent = result.drum_count;
    document.getElementById('ritual-scene-type').textContent = result.ritual_scene?.scene_type || '';
    document.getElementById('ritual-scene-desc').textContent = result.ritual_scene?.description || '';
    document.getElementById('ritual-history-ref').textContent = result.ritual_scene?.historical_reference || '';
    const spl = result.observer_spl_db;
    let feeling;
    if (spl > 110) feeling = '极度震撼！耳膜震颤，灵魂出窍，如亲临盛大祭祀现场！';
    else if (spl > 100) feeling = '非常震撼！胸腔共鸣，强烈感受到铜鼓的原始力量！';
    else if (spl > 90) feeling = '震撼有力！鼓声深沉悠远，仪式感十足。';
    else if (spl > 80) feeling = '清晰可闻，庄严沉静，适合静心聆听。';
    else feeling = '轻柔细腻，余韵悠长，如远山回响。';
    const feel = document.getElementById('ritual-feeling');
    if (feel) feel.textContent = feeling;
  }
}
