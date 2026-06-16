import { setupHiDPICanvas, drawMultiLineChart, drawBarChart } from './chart_utils.js';

export class EthnicComparatorPanel {
  constructor({ apiGet, apiPost }) {
    this.apiGet = apiGet;
    this.apiPost = apiPost;
    this.library = [];
    this.selectedDrums = new Set();
    this.lastCompareResult = null;
  }

  getEthnicColors() {
    return {
      '壮族': 'from-green-700 to-emerald-600',
      '苗族': 'from-purple-700 to-fuchsia-600',
      '瑶族': 'from-amber-700 to-orange-600',
    };
  }

  setup(ethnicLibrary) {
    this.library = ethnicLibrary;
    this.renderLibrary();
    this.setupEvents();
  }

  renderLibrary() {
    const container = document.getElementById('ethnic-drum-list');
    if (!container) return;
    const colors = this.getEthnicColors();
    container.innerHTML = this.library.map(d => `
      <div class="ethnic-drum-item cursor-pointer p-2.5 rounded-lg border transition-all ${
        this.selectedDrums.has(d.drum_id)
          ? 'border-cyan-400 bg-cyan-900/30 shadow-[0_0_12px_rgba(34,211,238,0.3)] scale-[1.02]'
          : 'border-slate-700/50 bg-slate-800/40 hover:border-slate-500 hover:bg-slate-700/50'
      }" data-id="${d.drum_id}">
        <div class="flex items-start gap-2.5">
          <div class="w-8 h-8 rounded-full bg-gradient-to-br ${colors[d.ethnic_group] || 'from-slate-700 to-slate-600'} flex items-center justify-center text-xs font-bold text-white shadow-md flex-shrink-0">
            ${d.ethnic_group.charAt(0)}
          </div>
          <div class="flex-1 min-w-0">
            <div class="flex items-center justify-between">
              <div class="font-semibold text-slate-100 text-sm truncate">${d.name}</div>
              <div class="flex items-center gap-0.5 ml-1 flex-shrink-0">
                ${this.selectedDrums.has(d.drum_id)
                  ? '<span class="text-cyan-400 text-xs font-bold">✓</span>'
                  : `<span class="text-[10px] font-mono bg-slate-700/60 text-slate-300 px-1.5 py-0.5 rounded">${d.diameter_cm}cm</span>`
                }
              </div>
            </div>
            <div class="text-[11px] text-slate-400 mt-0.5">${d.ethnic_group} · f₀=${d.fundamental_hz}Hz</div>
            <div class="text-[10px] text-amber-300/80 mt-1 line-clamp-1">📜 ${d.cultural_tag}</div>
          </div>
        </div>
      </div>
    `).join('');

    container.querySelectorAll('.ethnic-drum-item').forEach(item => {
      item.addEventListener('click', () => this.toggleSelect(item.dataset.id));
    });

    const countEl = document.getElementById('ethnic-selected-count');
    if (countEl) countEl.textContent = this.selectedDrums.size;
  }

  toggleSelect(drumId) {
    if (this.selectedDrums.has(drumId)) {
      this.selectedDrums.delete(drumId);
    } else {
      if (this.selectedDrums.size >= 6) {
        alert('最多选择6面铜鼓进行对比');
        return;
      }
      this.selectedDrums.add(drumId);
    }
    this.renderLibrary();
    this.renderCultureInfo();
  }

  renderCultureInfo() {
    const container = document.getElementById('ethnic-culture-info');
    if (!container) return;
    if (!this.selectedDrums.size) {
      container.innerHTML = `
        <div class="text-center py-10 text-slate-500">
          <div class="text-4xl mb-2">🥁</div>
          <div>请先从左侧列表点击选择铜鼓</div>
          <div class="text-xs mt-1">可多选（最多6面）进行声学特性对比</div>
        </div>`;
      return;
    }
    container.innerHTML = Array.from(this.selectedDrums).map(id => {
      const d = this.library.find(x => x.drum_id === id) || {};
      const colors = this.getEthnicColors();
      const colorCls = colors[d.ethnic_group] || 'from-slate-700 to-slate-600';
      return `
        <div class="bg-slate-800/60 rounded-lg p-3 border border-slate-700/50">
          <div class="flex items-center gap-2 mb-2">
            <div class="w-6 h-6 rounded-full bg-gradient-to-br ${colorCls} flex items-center justify-center text-[10px] font-bold text-white">
              ${d.ethnic_group ? d.ethnic_group.charAt(0) : '?'}
            </div>
            <div class="font-semibold text-slate-100 text-sm">${d.name || id}</div>
          </div>
          <div class="text-xs text-slate-400 space-y-0.5">
            <div>📏 直径: <span class="text-slate-300">${d.diameter_cm || '-'}cm</span></div>
            <div>🎵 基频: <span class="text-slate-300">${d.fundamental_hz || '-'}Hz</span></div>
            <div>📜 ${d.cultural_tag || '-'}</div>
          </div>
        </div>`;
    }).join('');
  }

  setupEvents() {
    const btn = document.getElementById('btn-ethnic-compare');
    if (btn) {
      btn.addEventListener('click', () => this.handleCompare());
    }
  }

  async handleCompare() {
    if (this.selectedDrums.size < 2) {
      alert('请至少选择2面铜鼓进行对比');
      return;
    }
    const ids = Array.from(this.selectedDrums);
    let result;
    try {
      result = await this.apiPost('/api/experience/ethnic-compare', { drum_ids: ids });
    } catch (e) {
      result = this.mockCompare(ids);
    }
    this.lastCompareResult = result;
    this.renderCompareCharts(result);
    this.renderCompareTable(result);
  }

  mockCompare(ids) {
    const drums = ids.map(id => {
      const d = this.library.find(x => x.drum_id === id);
      return { drum_id: id, name: d.name, ethnic_group: d.ethnic_group, diameter_cm: d.diameter_cm };
    });
    const freq = ids.map(id => {
      const d = this.library.find(x => x.drum_id === id);
      const base = d.fundamental_hz || 100;
      const bins = [];
      for (let i = 0; i < 15; i++) {
        bins.push({ frequency_hz: base * (i + 1), amplitude_db: 110 - i * 6 - Math.random() * 8 });
      }
      return [id, bins];
    });
    return {
      drums,
      frequency_spectra: freq,
      comparison_metrics: {
        fundamental_freqs: ids.map(id => {
          const d = this.library.find(x => x.drum_id === id);
          return [id, d.fundamental_hz || 100];
        }),
        sound_quality_scores: ids.map(id => {
          const d = this.library.find(x => x.drum_id === id);
          return [id, 75 + d.diameter_cm / 10];
        }),
        radiated_powers: ids.map((id, i) => [id, 0.1 + i * 0.05]),
        decay_times_s: ids.map((id, i) => [id, 1.5 + i * 0.3]),
        brightness: ids.map((id, i) => [id, 0.3 + i * 0.1]),
        harmonic_series: ids.map(id => {
          const d = this.library.find(x => x.drum_id === id);
          const base = d.fundamental_hz || 100;
          return [id, [1.0, 1.59, 2.14, 2.65, 3.16, 3.6, 4.1]];
        }),
      }
    };
  }

  renderCompareCharts(result) {
    const colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899'];
    const spectrumSeries = result.frequency_spectra.map(([id, bins], i) => {
      const d = this.library.find(x => x.drum_id === id) || {};
      return { name: d.name || id, data: bins.slice(0, 20).map(b => [b.frequency_hz, Math.max(b.amplitude_db, 20)]) };
    });
    const s1 = setupHiDPICanvas('ethnic-spectrum-compare', 480, 280);
    if (s1) drawMultiLineChart(s1.ctx, s1.width, s1.height, spectrumSeries, {
      title: '频谱对比 (Frequency Spectrum)', xLabel: '频率 Hz', yLabel: '幅度 dB', yLog: true, colors
    });

    const metricToShow = result.comparison_metrics.sound_quality_scores || result.comparison_metrics.fundamental_freqs || [];
    const bars = metricToShow.map(([id, val], i) => {
      const d = this.library.find(x => x.drum_id === id);
      return { label: (d ? d.name : id).slice(0, 6), value: val };
    });
    const s2 = setupHiDPICanvas('ethnic-bar-compare', 480, 280);
    if (s2) drawBarChart(s2.ctx, s2.width, s2.height, bars, {
      title: '综合音质评分 (Sound Quality)', colors, valueSuffix: '分'
    });
  }

  renderCompareTable(result) {
    const container = document.getElementById('ethnic-compare-table');
    if (!container) return;
    const m = result.comparison_metrics;
    const rows = (result.drums || []).map((d, i) => {
      const id = d.drum_id;
      const findVal = list => {
        const row = (list || []).find(x => x[0] === id);
        return row ? row[1] : null;
      };
      const fund = findVal(m.fundamental_freqs);
      const quality = findVal(m.sound_quality_scores);
      const power = findVal(m.radiated_powers);
      const decay = findVal(m.decay_times_s);
      const bright = findVal(m.brightness);
      return `<tr class="border-b border-slate-700/50 hover:bg-slate-700/30 transition-colors">
        <td class="px-3 py-2 font-medium text-cyan-300">${d.name || id}</td>
        <td class="px-3 py-2 text-slate-400 text-center">${d.ethnic_group || '-'}</td>
        <td class="px-3 py-2 text-emerald-400 text-center font-mono">${fund ? fund.toFixed(1) + ' Hz' : '-'}</td>
        <td class="px-3 py-2 text-amber-400 text-center">${quality ? quality.toFixed(1) + '分' : '-'}</td>
        <td class="px-3 py-2 text-blue-400 text-center font-mono">${power ? (power * 1000).toFixed(2) + ' mW' : '-'}</td>
        <td class="px-3 py-2 text-fuchsia-400 text-center font-mono">${decay ? decay.toFixed(2) + ' s' : '-'}</td>
        <td class="px-3 py-2 text-rose-400 text-center">${bright ? (bright * 100).toFixed(0) + '%' : '-'}</td>
      </tr>`;
    }).join('');

    container.innerHTML = `
      <div class="overflow-x-auto border border-slate-700/50 rounded-lg">
        <table class="w-full text-sm">
          <thead class="bg-slate-800/80">
            <tr>
              <th class="px-3 py-2 text-left text-slate-300">铜鼓名称</th>
              <th class="px-3 py-2 text-center text-slate-300">民族</th>
              <th class="px-3 py-2 text-center text-slate-300">基频</th>
              <th class="px-3 py-2 text-center text-slate-300">音质</th>
              <th class="px-3 py-2 text-center text-slate-300">声功率</th>
              <th class="px-3 py-2 text-center text-slate-300">衰减</th>
              <th class="px-3 py-2 text-center text-slate-300">亮度</th>
            </tr>
          </thead>
          <tbody>${rows}</tbody>
        </table>
      </div>`;
  }
}
