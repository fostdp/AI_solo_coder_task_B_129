export class ExperiencePanel {
  constructor() {
    this.selectedDrums = new Set();
    this.ethnicLibrary = [];
    this.currentDrumId = 'zhuang-dagu';
    this.audioCtx = null;
    this.lastTapTime = 0;
  }

  async init() {
    await this.loadEthnicLibrary();
    this.setupMainTabs();
    this.setupExpTabs();
    this.setupEthnicCompare();
    this.setupCrossEra();
    this.setupRitual();
    this.setupPlayMode();
  }

  apiBase() {
    if (window.location.port === '3000' || window.location.protocol === 'file:') return 'http://localhost:8080';
    return '';
  }

  async apiGet(path) {
    const res = await fetch(this.apiBase() + path);
    const data = await res.json();
    if (!data.success) throw new Error(data.message || 'API error');
    return data.data;
  }

  async apiPost(path, body) {
    const res = await fetch(this.apiBase() + path, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    });
    const data = await res.json();
    if (!data.success) throw new Error(data.message || 'API error');
    return data.data;
  }

  setupMainTabs() {
    const tabs = document.querySelectorAll('.main-tab-btn');
    tabs.forEach(tab => {
      tab.addEventListener('click', () => {
        const target = tab.dataset.mainTab;
        tabs.forEach(t => {
          t.classList.remove('active', 'border-slate-700');
          t.classList.add('bg-slate-800/50', 'border-slate-700/50', 'text-slate-400');
        });
        tab.classList.add('active', 'border-slate-700');
        tab.classList.remove('bg-slate-800/50', 'border-slate-700/50', 'text-slate-400');

        document.querySelectorAll('.main-tab-content').forEach(c => c.classList.add('hidden'));
        document.getElementById(`${target}-view`).classList.remove('hidden');

        if (target === 'experience' && !this.experienceInited) {
          this.experienceInited = true;
          setTimeout(() => this.redrawCanvases(), 100);
        }
      });
    });
  }

  setupExpTabs() {
    const tabs = document.querySelectorAll('.exp-tab-btn');
    tabs.forEach(tab => {
      tab.addEventListener('click', () => {
        const target = tab.dataset.expTab;
        tabs.forEach(t => {
          t.classList.remove('active', 'border-amber-600/50', 'text-amber-300');
          t.classList.add('bg-slate-800/50', 'border-slate-700', 'text-slate-400');
        });
        tab.classList.add('active', 'border-amber-600/50', 'text-amber-300');
        tab.classList.remove('bg-slate-800/50', 'border-slate-700', 'text-slate-400');

        document.querySelectorAll('.exp-tab-content').forEach(c => c.classList.add('hidden'));
        document.getElementById(`exp-${target}`).classList.remove('hidden');

        setTimeout(() => this.redrawCanvases(), 50);
      });
    });
  }

  redrawCanvases() {
    this.drawEthnicEmpty();
    this.drawCrossEraEmpty();
    this.drawRitualEmpty();
    this.drawPlayEmpty();
  }

  async loadEthnicLibrary() {
    try {
      this.ethnicLibrary = await this.apiGet('/api/experience/ethnic-library');
      this.renderEthnicList();
      this.populateSelects();
    } catch (e) {
      console.warn('Failed to load ethnic library, using mock data');
      this.ethnicLibrary = [
        { drum_id: 'zhuang-dagu', name: '壮族大铜鼓', ethnic_group: '壮族', diameter_cm: 100, fundamental_hz: 98, cultural_tag: '蛙图腾·雷王鼓' },
        { drum_id: 'zhuang-magu', name: '壮族麻江型铜鼓', ethnic_group: '壮族', diameter_cm: 50, fundamental_hz: 210, cultural_tag: '丧葬·祭祀' },
        { drum_id: 'miao-dagu', name: '苗族大铜鼓', ethnic_group: '苗族', diameter_cm: 80, fundamental_hz: 130, cultural_tag: '牯藏节·祭祖' },
        { drum_id: 'miao-xiaogu', name: '苗族小铜鼓', ethnic_group: '苗族', diameter_cm: 35, fundamental_hz: 310, cultural_tag: '跳月·芦笙舞' },
        { drum_id: 'yao-dagu', name: '瑶族大铜鼓', ethnic_group: '瑶族', diameter_cm: 70, fundamental_hz: 150, cultural_tag: '盘王节·长鼓舞' },
        { drum_id: 'yao-guzai', name: '瑶族铜鼓仔', ethnic_group: '瑶族', diameter_cm: 25, fundamental_hz: 440, cultural_tag: '耍歌堂·喜庆' },
      ];
      this.renderEthnicList();
      this.populateSelects();
    }
  }

  populateSelects() {
    const selects = ['crossera-ancient-select', 'ritual-drum-type', 'play-drum-select'];
    selects.forEach(id => {
      const el = document.getElementById(id);
      if (!el) return;
      el.innerHTML = this.ethnicLibrary.map(d =>
        `<option value="${d.drum_id}">${d.name} (${d.diameter_cm}cm)</option>`
      ).join('');
    });
  }

  renderEthnicList() {
    const container = document.getElementById('ethnic-drum-list');
    if (!container) return;

    const ethnicColors = {
      '壮族': 'from-emerald-900 to-emerald-700',
      '苗族': 'from-indigo-900 to-indigo-700',
      '瑶族': 'from-rose-900 to-rose-700',
    };

    container.innerHTML = this.ethnicLibrary.map(d => `
      <div class="ethnic-drum-item cursor-pointer p-2.5 rounded-lg border transition-all ${
        this.selectedDrums.has(d.drum_id)
          ? 'bg-amber-900/40 border-amber-500/60'
          : 'bg-slate-700/40 border-slate-600/50 hover:border-amber-600/40'
      }" data-drum-id="${d.drum_id}">
        <div class="flex items-center gap-2">
          <div class="w-8 h-8 rounded-full bg-gradient-to-br ${ethnicColors[d.ethnic_group] || 'from-slate-700 to-slate-600'} flex items-center justify-center text-xs">
            ${d.ethnic_group.charAt(0)}
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium truncate">${d.name}</div>
            <div class="text-xs text-slate-400">${d.diameter_cm}cm · ${d.fundamental_hz}Hz</div>
          </div>
          <div class="text-xs ${this.selectedDrums.has(d.drum_id) ? 'text-amber-400' : 'text-slate-500'}">
            ${this.selectedDrums.has(d.drum_id) ? '✓' : ''}
          </div>
        </div>
        <div class="text-xs text-amber-600/80 mt-1 truncate">${d.cultural_tag}</div>
      </div>
    `).join('');

    container.querySelectorAll('.ethnic-drum-item').forEach(item => {
      item.addEventListener('click', () => {
        const id = item.dataset.drumId;
        if (this.selectedDrums.has(id)) {
          this.selectedDrums.delete(id);
        } else {
          if (this.selectedDrums.size >= 4) {
            alert('最多选择4面铜鼓进行对比');
            return;
          }
          this.selectedDrums.add(id);
        }
        document.getElementById('ethnic-selected-count').textContent = this.selectedDrums.size;
        this.renderEthnicList();
        this.updateEthnicCultureInfo();
      });
    });
  }

  updateEthnicCultureInfo() {
    const container = document.getElementById('ethnic-culture-info');
    if (!container) return;

    if (this.selectedDrums.size === 0) {
      container.innerHTML = '<p class="text-slate-500">选择铜鼓查看民族文化信息</p>';
      return;
    }

    const cultureInfo = {
      'zhuang-dagu': {
        group: '壮族',
        text: '壮族是最早铸造和使用铜鼓的民族之一。大铜鼓被视为雷王的化身，蛙纹象征雨水与丰收。每年"蚂拐节"中，铜鼓是核心礼器。',
        festivals: ['蚂拐节', '三月三', '丰收祭']
      },
      'zhuang-magu': {
        group: '壮族',
        text: '麻江型铜鼓体型小巧，纹饰精美，是民间使用最广泛的类型。婚丧嫁娶、节日庆典都离不开它。',
        festivals: ['婚嫁', '丧葬', '牛魂节']
      },
      'miao-dagu': {
        group: '苗族',
        text: '苗族铜鼓是牯藏节的神圣礼器，每十三年举办一次。鼓声被认为能够沟通祖先，护送灵魂回到祖先故地。',
        festivals: ['牯藏节', '苗年', '吃新节']
      },
      'miao-xiaogu': {
        group: '苗族',
        text: '小型铜鼓是苗族青年社交活动的重要乐器。跳月时，铜鼓与芦笙合奏，青年男女翩翩起舞。',
        festivals: ['跳月', '芦笙会', '姐妹饭节']
      },
      'yao-dagu': {
        group: '瑶族',
        text: '瑶族铜鼓与长鼓并称盘王节两大乐器。铜鼓指挥节奏，长鼓随之起舞，是瑶族祭祀盘王的传统形式。',
        festivals: ['盘王节', '耍歌堂', '达努节']
      },
      'yao-guzai': {
        group: '瑶族',
        text: '铜鼓仔音色明亮清脆，是瑶族儿童喜爱的玩具，也是青年男女对歌时的伴奏乐器。',
        festivals: ['耍歌堂', '情歌对唱', '儿童游戏']
      }
    };

    const firstId = Array.from(this.selectedDrums)[0];
    const info = cultureInfo[firstId] || { group: '未知', text: '', festivals: [] };

    container.innerHTML = `
      <div class="font-semibold text-amber-300 mb-1">${info.group}铜鼓文化</div>
      <p class="text-slate-300 leading-relaxed">${info.text}</p>
      <div class="mt-2 flex flex-wrap gap-1">
        ${info.festivals.map(f => `<span class="text-xs px-2 py-0.5 bg-amber-900/40 rounded text-amber-300">${f}</span>`).join('')}
      </div>
    `;
  }

  setupEthnicCompare() {
    const btn = document.getElementById('btn-ethnic-compare');
    if (btn) {
      btn.addEventListener('click', () => this.runEthnicCompare());
    }
  }

  async runEthnicCompare() {
    if (this.selectedDrums.size < 2) {
      alert('请至少选择2面铜鼓进行对比');
      return;
    }

    try {
      const result = await this.apiPost('/api/experience/ethnic-compare', {
        drum_ids: Array.from(this.selectedDrums)
      });
      this.drawEthnicSpectrumCompare(result);
      this.drawEthnicBarCompare(result);
      this.updateEthnicCompareTable(result);
    } catch (e) {
      console.error('Ethnic compare failed:', e);
      this.mockEthnicCompare();
    }
  }

  mockEthnicCompare() {
    const mockData = {
      drums: Array.from(this.selectedDrums).map(id => {
        const d = this.ethnicLibrary.find(x => x.drum_id === id);
        return { drum_id: id, name: d.name, ethnic_group: d.ethnic_group, diameter_cm: d.diameter_cm };
      }),
      comparison_metrics: {
        fundamental_freqs: Array.from(this.selectedDrums).map(id => {
          const d = this.ethnicLibrary.find(x => x.drum_id === id);
          return [id, d.fundamental_hz];
        }),
        sound_quality_scores: Array.from(this.selectedDrums).map((id, i) => [id, 0.7 + i * 0.05]),
        radiated_powers: Array.from(this.selectedDrums).map(id => {
          const d = this.ethnicLibrary.find(x => x.drum_id === id);
          return [id, 0.5 + d.diameter_cm * 0.01];
        }),
        brightness: Array.from(this.selectedDrums).map(id => {
          const d = this.ethnicLibrary.find(x => x.drum_id === id);
          return [id, 3 + d.diameter_cm * 0.02];
        }),
        decay_times_s: Array.from(this.selectedDrums).map((id, i) => [id, 2 + i * 0.3]),
      },
      frequency_spectra: Array.from(this.selectedDrums).map(id => {
        const d = this.ethnicLibrary.find(x => x.drum_id === id);
        const spectrum = [];
        for (let f = 20; f <= 2000; f += 5) {
          let amp = 0;
          for (let n = 1; n <= 6; n++) {
            const freq = d.fundamental_hz * (n * 1.5);
            const width = freq * 0.02 + 3;
            const diff = Math.abs(f - freq);
            if (diff < width * 3) {
              const gauss = Math.exp(-(diff * diff) / (2 * width * width));
              amp += gauss / Math.sqrt(n);
            }
          }
          spectrum.push({ frequency_hz: f, amplitude_db: amp > 0 ? 20 * Math.log10(amp) + 80 : 0 });
        }
        return [id, spectrum];
      })
    };
    this.drawEthnicSpectrumCompare(mockData);
    this.drawEthnicBarCompare(mockData);
    this.updateEthnicCompareTable(mockData);
  }

  drawEthnicEmpty() {
    const canvases = ['ethnic-spectrum-compare', 'ethnic-bar-compare',
      'crossera-spectrum', 'ritual-soundfield-top', 'ritual-envelope',
      'play-drum-surface', 'play-spectrum', 'play-envelope'];
    canvases.forEach(id => {
      const c = document.getElementById(id);
      if (!c) return;
      const ctx = c.getContext('2d');
      ctx.clearRect(0, 0, c.width, c.height);
    });
  }

  drawEthnicSpectrumCompare(result) {
    const canvas = document.getElementById('ethnic-spectrum-compare');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    ctx.scale(dpr, dpr);

    const colors = ['#f59e0b', '#3b82f6', '#10b981', '#f43f5e'];
    const labels = result.frequency_spectra.map(s => s[0]);

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    ctx.strokeStyle = '#1e293b';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 5; i++) {
      const y = (h - 40) - i * (h - 60) / 5 + 20;
      ctx.beginPath();
      ctx.moveTo(50, y);
      ctx.lineTo(w - 20, y);
      ctx.stroke();
      ctx.fillStyle = '#64748b';
      ctx.font = '10px sans-serif';
      ctx.textAlign = 'right';
      ctx.fillText(`${20 + i * 20}`, 45, y + 3);
    }

    const maxF = 2000;
    for (let f = 250; f <= maxF; f += 250) {
      const x = 50 + (f / maxF) * (w - 70);
      ctx.beginPath();
      ctx.moveTo(x, 20);
      ctx.lineTo(x, h - 40);
      ctx.stroke();
      ctx.fillStyle = '#64748b';
      ctx.font = '10px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(`${f}Hz`, x, h - 25);
    }

    result.frequency_spectra.forEach(([drumId, spectrum], idx) => {
      const color = colors[idx % colors.length];
      const drumInfo = result.drums.find(d => d.drum_id === drumId);
      const name = drumInfo ? drumInfo.name : drumId;

      ctx.beginPath();
      ctx.strokeStyle = color;
      ctx.lineWidth = 2;

      spectrum.forEach((bin, i) => {
        const f = bin.frequency_hz;
        const amp = bin.amplitude_db;
        const x = 50 + Math.max(0, Math.min(1, (f - 20) / (maxF - 20))) * (w - 70);
        const y = (h - 40) - Math.max(0, Math.min(1, (amp - 20) / 60)) * (h - 60) + 20;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      });
      ctx.stroke();

      const legendY = 8 + idx * 16;
      ctx.fillStyle = color;
      ctx.fillRect(w - 180, legendY, 12, 3);
      ctx.fillStyle = '#cbd5e1';
      ctx.font = '11px sans-serif';
      ctx.textAlign = 'left';
      ctx.fillText(name, w - 163, legendY + 6);
    });
  }

  drawEthnicBarCompare(result) {
    const canvas = document.getElementById('ethnic-bar-compare');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    ctx.scale(dpr, dpr);

    const colors = ['#f59e0b', '#3b82f6', '#10b981', '#f43f5e'];

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    const metrics = [
      { label: '基频(Hz)', data: result.comparison_metrics.fundamental_freqs },
      { label: '品质(分)', data: result.comparison_metrics.sound_quality_scores.map(([id, v]) => [id, v * 100]) },
      { label: '声功率(W)', data: result.comparison_metrics.radiated_powers },
    ];

    const numBars = result.drums.length;
    const groupWidth = (w - 80) / metrics.length;

    metrics.forEach((metric, mi) => {
      const gx = 60 + mi * groupWidth + groupWidth * 0.1;
      const gw = groupWidth * 0.8 / numBars;

      const vals = metric.data.map(([id, v]) => v);
      const maxVal = Math.max(...vals) * 1.1;

      metric.data.forEach(([id, val], di) => {
        const bx = gx + di * gw;
        const bh = (val / maxVal) * (h - 80);
        const by = h - 50 - bh;

        ctx.fillStyle = colors[di % colors.length];
        ctx.fillRect(bx + 2, by, gw - 4, bh);

        ctx.fillStyle = '#e2e8f0';
        ctx.font = '10px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(val.toFixed(1), bx + gw / 2, by - 4);
      });

      ctx.fillStyle = '#94a3b8';
      ctx.font = '11px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(metric.label, gx + groupWidth * 0.4, h - 25);
    });

    const legendY = 5;
    result.drums.forEach((drum, i) => {
      ctx.fillStyle = colors[i % colors.length];
      ctx.fillRect(10 + i * 90, legendY, 10, 3);
      ctx.fillStyle = '#cbd5e1';
      ctx.font = '10px sans-serif';
      ctx.textAlign = 'left';
      ctx.fillText(drum.name.substring(0, 6), 24 + i * 90, legendY + 6);
    });
  }

  updateEthnicCompareTable(result) {
    const container = document.getElementById('ethnic-compare-table');
    if (!container) return;

    const metrics = result.comparison_metrics;

    const rows = [
      ['指标', ...result.drums.map(d => d.name)],
      ['基频 (Hz)', ...metrics.fundamental_freqs.map(([_, v]) => v.toFixed(1))],
      ['品质评分', ...metrics.sound_quality_scores.map(([_, v]) => (v * 100).toFixed(1))],
      ['声功率 (W)', ...metrics.radiated_powers.map(([_, v]) => v.toFixed(3))],
      ['衰减时间 (s)', ...metrics.decay_times_s.map(([_, v]) => v.toFixed(2))],
      ['亮度系数', ...metrics.brightness.map(([_, v]) => v.toFixed(2))],
    ];

    container.innerHTML = `
      <table class="w-full text-xs">
        <tbody>
          ${rows.map((row, ri) => `
            <tr class="${ri % 2 === 0 ? 'bg-slate-700/30' : ''}">
              ${row.map((cell, ci) => `
                <td class="px-2 py-1.5 ${ci === 0 ? 'text-slate-400 font-medium' : 'text-slate-200 text-center'}">${cell}</td>
              `).join('')}
            </tr>
          `).join('')}
        </tbody>
      </table>
    `;
  }

  setupCrossEra() {
    const ancientSelect = document.getElementById('crossera-ancient-select');
    if (ancientSelect) {
      ancientSelect.addEventListener('change', () => {
        const drumId = ancientSelect.value;
        this.updateAncientInfo(drumId);
      });
      setTimeout(() => this.updateAncientInfo(ancientSelect.value), 100);
    }

    const modernSelect = document.getElementById('crossera-modern-select');
    if (modernSelect) {
      modernSelect.addEventListener('change', () => {
        const size = parseFloat(modernSelect.value);
        this.updateModernInfo(size);
      });
      setTimeout(() => this.updateModernInfo(parseFloat(modernSelect.value)), 100);
    }

    const btn = document.getElementById('btn-crossera-compare');
    if (btn) {
      btn.addEventListener('click', () => this.runCrossEraCompare());
    }
  }

  async updateAncientInfo(drumId) {
    const drum = this.ethnicLibrary.find(d => d.drum_id === drumId);
    if (!drum) return;

    try {
      const profile = await this.apiGet(`/api/experience/ethnic-drum/${drumId}`);
      document.getElementById('crossera-ancient-ethnic').textContent = profile.ethnic_group;
      document.getElementById('crossera-ancient-era').textContent = profile.estimated_era;
      document.getElementById('crossera-ancient-diam').textContent = `${profile.diameter_cm}cm`;
    } catch {
      document.getElementById('crossera-ancient-ethnic').textContent = drum.ethnic_group;
      document.getElementById('crossera-ancient-era').textContent = '古代';
      document.getElementById('crossera-ancient-diam').textContent = `${drum.diameter_cm}cm`;
    }
  }

  updateModernInfo(sizeInches) {
    const fundamental = 523.25 * Math.sqrt(29 / sizeInches);
    const range = `${(fundamental * 0.75).toFixed(0)}-${(fundamental * 1.33).toFixed(0)}Hz`;

    document.getElementById('crossera-modern-mat').textContent = '聚酯薄膜鼓皮+铜锅';
    document.getElementById('crossera-modern-range').textContent = range;
  }

  async runCrossEraCompare() {
    const ancientId = document.getElementById('crossera-ancient-select').value;
    const timpaniSize = parseFloat(document.getElementById('crossera-modern-select').value);

    try {
      const result = await this.apiPost('/api/experience/cross-era', {
        drum_id: ancientId,
        timpani_size_inches: timpaniSize
      });
      this.drawCrossEraSpectrum(result);
      this.updateCrossEraMetrics(result);
    } catch (e) {
      console.error('Cross-era compare failed:', e);
      this.mockCrossEraCompare(ancientId, timpaniSize);
    }
  }

  mockCrossEraCompare(ancientId, timpaniSize) {
    const ancient = this.ethnicLibrary.find(d => d.drum_id === ancientId);
    const modernFund = 523.25 * Math.sqrt(29 / timpaniSize);

    const ancientSpec = [];
    const modernSpec = [];
    for (let f = 20; f <= 3000; f += 5) {
      let aAmp = 0, mAmp = 0;
      for (let n = 1; n <= 6; n++) {
        const aFreq = ancient.fundamental_hz * (n * 1.6 + 0.2);
        const aW = aFreq * 0.02 + 3;
        const aDiff = Math.abs(f - aFreq);
        if (aDiff < aW * 3) {
          aAmp += Math.exp(-(aDiff * aDiff) / (2 * aW * aW)) / Math.sqrt(n);
        }

        const mRatios = [1, 1.59, 2, 2.45, 2.9, 3.4];
        const mFreq = modernFund * mRatios[n - 1];
        const mW = mFreq * 0.01 + 2;
        const mDiff = Math.abs(f - mFreq);
        if (mDiff < mW * 3) {
          mAmp += Math.exp(-(mDiff * mDiff) / (2 * mW * mW)) / Math.sqrt(n);
        }
      }
      ancientSpec.push({ frequency_hz: f, amplitude_db: aAmp > 0 ? 20 * Math.log10(aAmp) + 80 : 0 });
      modernSpec.push({ frequency_hz: f, amplitude_db: mAmp > 0 ? 20 * Math.log10(mAmp) + 85 : 0 });
    }

    const result = {
      ancient_drum: { drum_id: ancientId, name: ancient.name },
      modern_drum: { name: `${timpaniSize}" 定音鼓` },
      metrics_comparison: {
        ancient_fundamental_hz: ancient.fundamental_hz,
        modern_fundamental_hz: modernFund,
        ancient_decay_s: 2.5,
        modern_decay_s: 1.2,
        ancient_brightness: 65,
        modern_brightness: 85,
        ancient_radiated_power_w: 0.8,
        modern_radiated_power_w: 0.5,
        pitch_tunability: '铜鼓：固定音高；定音鼓：可调音高',
        cultural_context: '铜鼓是礼器乐器合一，沟通天地人神；定音鼓是交响乐队核心打击乐器。'
      },
      ancient_spectrum: ancientSpec,
      modern_spectrum: modernSpec
    };

    this.drawCrossEraSpectrum(result);
    this.updateCrossEraMetrics(result);
  }

  drawCrossEraSpectrum(result) {
    const canvas = document.getElementById('crossera-spectrum');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    ctx.scale(dpr, dpr);

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    ctx.strokeStyle = '#1e293b';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 5; i++) {
      const y = (h - 40) - i * (h - 60) / 5 + 20;
      ctx.beginPath();
      ctx.moveTo(50, y);
      ctx.lineTo(w - 20, y);
      ctx.stroke();
    }

    const maxF = 3000;
    const drawSpec = (spectrum, color, dashed) => {
      ctx.beginPath();
      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      if (dashed) ctx.setLineDash([6, 4]);
      spectrum.forEach((bin, i) => {
        const f = bin.frequency_hz;
        const amp = bin.amplitude_db;
        const x = 50 + Math.max(0, Math.min(1, (f - 20) / (maxF - 20))) * (w - 70);
        const y = (h - 40) - Math.max(0, Math.min(1, (amp - 20) / 70)) * (h - 60) + 20;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      });
      ctx.stroke();
      ctx.setLineDash([]);
    };

    drawSpec(result.ancient_spectrum, '#f59e0b', false);
    drawSpec(result.modern_spectrum, '#6366f1', true);

    const legendY = 8;
    ctx.fillStyle = '#f59e0b';
    ctx.fillRect(w - 220, legendY, 14, 3);
    ctx.fillStyle = '#e2e8f0';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'left';
    ctx.fillText(`古代：${result.ancient_drum.name}`, w - 200, legendY + 6);

    ctx.strokeStyle = '#6366f1';
    ctx.setLineDash([6, 4]);
    ctx.beginPath();
    ctx.moveTo(w - 220, legendY + 22);
    ctx.lineTo(w - 206, legendY + 22);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = '#e2e8f0';
    ctx.fillText(`现代：${result.modern_drum.name}`, w - 200, legendY + 26);
  }

  updateCrossEraMetrics(result) {
    const m = result.metrics_comparison;
    document.getElementById('crossera-ancient-freq').textContent = m.ancient_fundamental_hz.toFixed(1) + 'Hz';
    document.getElementById('crossera-modern-freq').textContent = m.modern_fundamental_hz.toFixed(1) + 'Hz';
    document.getElementById('crossera-ratio').textContent = (m.ancient_fundamental_hz / m.modern_fundamental_hz).toFixed(2);
    document.getElementById('crossera-cultural-note').textContent = m.cultural_context;
  }

  setupRitual() {
    const countSlider = document.getElementById('ritual-drum-count');
    const radiusSlider = document.getElementById('ritual-radius');
    if (countSlider) {
      countSlider.addEventListener('input', () => {
        document.getElementById('ritual-count-val').textContent = countSlider.value;
      });
    }
    if (radiusSlider) {
      radiusSlider.addEventListener('input', () => {
        document.getElementById('ritual-radius-val').textContent = radiusSlider.value;
      });
    }

    const btn = document.getElementById('btn-ritual-simulate');
    if (btn) {
      btn.addEventListener('click', () => this.runRitual());
    }
  }

  generateDrumPlacements(pattern, count, radius, drumId) {
    const placements = [];
    for (let i = 0; i < count; i++) {
      let x, y;
      switch (pattern) {
        case 'circle':
          const angle = (i / count) * Math.PI * 2;
          x = Math.cos(angle) * radius;
          y = Math.sin(angle) * radius;
          break;
        case 'semicircle':
          const sa = (i / (count - 1 || 1)) * Math.PI;
          x = Math.cos(sa - Math.PI / 2) * radius;
          y = Math.abs(Math.sin(sa - Math.PI / 2) * radius);
          break;
        case 'line':
          x = (i - (count - 1) / 2) * (radius / count * 2);
          y = 0;
          break;
        case 'square':
          const sideCount = Math.ceil(count / 4);
          const side = Math.floor(i / sideCount);
          const pos = i % sideCount;
          const half = radius;
          const step = (radius * 2) / (sideCount - 1 || 1);
          if (side === 0) { x = -half + pos * step; y = -half; }
          else if (side === 1) { x = half; y = -half + pos * step; }
          else if (side === 2) { x = half - pos * step; y = half; }
          else { x = -half; y = half - pos * step; }
          break;
      }
      placements.push({
        drum_id: drumId,
        position_x_m: x,
        position_y_m: y,
        position_z_m: 0,
        relative_volume: 1.0,
        strike_phase_offset_s: i * 0.02
      });
    }
    return placements;
  }

  async runRitual() {
    const pattern = document.getElementById('ritual-pattern').value;
    const count = parseInt(document.getElementById('ritual-drum-count').value);
    const radius = parseFloat(document.getElementById('ritual-radius').value);
    const drumType = document.getElementById('ritual-drum-type').value;

    const placements = this.generateDrumPlacements(pattern, count, radius, drumType);

    try {
      const result = await this.apiPost('/api/experience/ritual-soundfield', {
        drum_placements: placements,
        observer_x_m: 0,
        observer_y_m: 0,
        observer_z_m: 1.5,
        field_radius_m: radius * 1.5,
        grid_resolution: 16
      });
      this.drawRitualSoundField(result, placements);
      this.updateRitualInfo(result);
    } catch (e) {
      console.error('Ritual simulation failed:', e);
      this.mockRitualResult(placements, radius);
    }
  }

  mockRitualResult(placements, radius) {
    const soundField = [];
    const gridR = radius * 1.5;
    const res = 20;
    for (let i = 0; i < res; i++) {
      for (let j = 0; j <= res / 2; j++) {
        const theta = (i / res) * Math.PI * 2;
        const phi = (j / (res / 2)) * Math.PI / 2;
        const x = gridR * Math.cos(phi) * Math.cos(theta);
        const y = gridR * Math.cos(phi) * Math.sin(theta);
        const z = gridR * Math.sin(phi);

        let intensity = 0;
        placements.forEach(p => {
          const dx = x - p.position_x_m;
          const dy = y - p.position_y_m;
          const dz = z - p.position_z_m;
          const dist = Math.sqrt(dx * dx + dy * dy + dz * dz);
          intensity += 0.5 * p.relative_volume / (4 * Math.PI * dist * dist + 1);
        });

        const spl = intensity > 1e-12 ? 10 * Math.log10(intensity / 1e-12) : 0;
        soundField.push({ x_m: x, y_m: y, z_m: z, spl_db: spl });
      }
    }

    const envelope = [];
    for (let t = 0; t < 100; t++) {
      const time = t * 0.03;
      let amp = 0;
      placements.forEach(p => {
        const dt = time - p.strike_phase_offset_s;
        if (dt >= 0) {
          amp += Math.exp(-dt / 1.5) * (1 - Math.exp(-dt / 0.02)) * p.relative_volume;
        }
      });
      envelope.push([time, amp]);
    }

    const result = {
      sound_field: soundField,
      observer_spl_db: 85 + Math.sqrt(placements.length) * 8,
      total_radiated_power_w: placements.length * 0.6,
      drum_count: placements.length,
      temporal_envelope: envelope,
      ritual_scene: {
        scene_type: this.getSceneType(placements.length),
        description: this.getSceneDesc(placements.length),
        historical_reference: '《后汉书·马援传》：得铜鼓，鼓高大者为贵，群情慑服。'
      }
    };

    this.drawRitualSoundField(result, placements);
    this.updateRitualInfo(result);
  }

  getSceneType(count) {
    if (count === 1) return '独奏祭祀';
    if (count <= 4) return '四方祭祀阵';
    if (count <= 8) return '八方祭祀阵';
    return '百鼓齐鸣大典';
  }

  getSceneDesc(count) {
    if (count === 1) return '单鼓独奏是最古老的铜鼓使用形式，通常由巫师或族长在祭祀中敲击，沟通神灵。';
    if (count <= 4) return '四面铜鼓按东南西北四方布阵，象征四象四季，是中等规模祭祀的典型配置。';
    if (count <= 8) return '八面对称排列的铜鼓大阵，用于重大祭祀仪式，声音可达十里之外。';
    return '数十面上百面铜鼓同时敲击，是民族地区最盛大的节庆场景，震撼人心。';
  }

  drawRitualSoundField(result, placements) {
    const canvas = document.getElementById('ritual-soundfield-top');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth || canvas.width || 400;
    const h = canvas.clientHeight || canvas.height || 400;
    const size = Math.max(200, Math.min(w, h));
    canvas.width = size * dpr;
    canvas.height = size * dpr;
    ctx.scale(dpr, dpr);

    const cx = size / 2;
    const cy = size / 2;
    const viewRadius = Math.max(80, Math.min(w, h) * 0.45);

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, size, size);

    const gridSize = 50;
    const gridStep = size / gridSize;

    const maxDist = viewRadius;

    for (let i = 0; i < gridSize; i++) {
      for (let j = 0; j < gridSize; j++) {
        const gx = i * gridStep;
        const gy = j * gridStep;
        const dx = (gx - cx) / viewRadius * maxDist;
        const dy = (gy - cy) / viewRadius * maxDist;
        const distFromCenter = Math.sqrt(dx * dx + dy * dy);
        if (distFromCenter > maxDist) continue;

        let intensity = 0;
        placements.forEach(p => {
          const pdx = dx - p.position_x_m;
          const pdy = dy - p.position_y_m;
          const d = Math.sqrt(pdx * pdx + pdy * pdy);
          intensity += 0.5 * p.relative_volume / (4 * Math.PI * (d * d + 0.5));
        });

        const spl = intensity > 1e-12 ? 10 * Math.log10(intensity / 1e-12) : 0;
        const norm = Math.max(0, Math.min(1, (spl - 60) / 40));

        const hue = 240 - norm * 200;
        const sat = 80;
        const light = 20 + norm * 40;
        ctx.fillStyle = `hsla(${hue}, ${sat}%, ${light}%, 0.85)`;
        ctx.fillRect(gx, gy, gridStep + 0.5, gridStep + 0.5);
      }
    }

    placements.forEach(p => {
      const px = cx + p.position_x_m / maxDist * viewRadius;
      const py = cy + p.position_y_m / maxDist * viewRadius;

      const gradient = ctx.createRadialGradient(px, py, 0, px, py, 12);
      gradient.addColorStop(0, 'rgba(245, 158, 11, 0.9)');
      gradient.addColorStop(1, 'rgba(245, 158, 11, 0)');
      ctx.fillStyle = gradient;
      ctx.beginPath();
      ctx.arc(px, py, 12, 0, Math.PI * 2);
      ctx.fill();

      ctx.fillStyle = '#fbbf24';
      ctx.beginPath();
      ctx.arc(px, py, 4, 0, Math.PI * 2);
      ctx.fill();
    });

    ctx.fillStyle = '#ef4444';
    ctx.beginPath();
    ctx.arc(cx, cy, 5, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#fca5a5';
    ctx.font = '10px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('观察者', cx, cy + 18);

    this.drawRitualEnvelope(result);
  }

  drawRitualEnvelope(result) {
    const canvas = document.getElementById('ritual-envelope');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth || canvas.width || 400;
    const h = canvas.clientHeight || canvas.height || 120;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    ctx.scale(dpr, dpr);

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    const envelope = result.temporal_envelope;
    if (!envelope || envelope.length === 0) return;

    const maxAmp = Math.max(...envelope.map(e => e[1]));
    if (maxAmp === 0) return;

    ctx.beginPath();
    ctx.strokeStyle = '#f59e0b';
    ctx.lineWidth = 2;
    envelope.forEach(([t, a], i) => {
      const x = 30 + (t / 3) * (w - 40);
      const y = h - 20 - (a / maxAmp) * (h - 40);
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.stroke();

    ctx.fillStyle = 'rgba(245, 158, 11, 0.2)';
    ctx.beginPath();
    envelope.forEach(([t, a], i) => {
      const x = 30 + (t / 3) * (w - 40);
      const y = h - 20 - (a / maxAmp) * (h - 40);
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.lineTo(30 + (envelope[envelope.length - 1][0] / 3) * (w - 40), h - 20);
    ctx.lineTo(30, h - 20);
    ctx.closePath();
    ctx.fill();
  }

  updateRitualInfo(result) {
    document.getElementById('ritual-total-power').textContent = result.total_radiated_power_w.toFixed(2) + 'W';
    document.getElementById('ritual-center-spl').textContent = result.observer_spl_db.toFixed(1) + 'dB';
    document.getElementById('ritual-drum-num').textContent = result.drum_count;
    document.getElementById('ritual-scene-type').textContent = result.ritual_scene.scene_type;
    document.getElementById('ritual-scene-desc').textContent = result.ritual_scene.description;
    document.getElementById('ritual-history-ref').textContent = result.ritual_scene.historical_reference;

    const spl = result.observer_spl_db;
    let feeling = '';
    if (spl < 70) feeling = '声音清晰柔和，如远处传来的仪式声，可作背景音乐感受。';
    else if (spl < 85) feeling = '声音洪亮饱满，能感受到铜鼓的共振，身临其境般的仪式氛围。';
    else if (spl < 100) feeling = '声音震撼有力，胸腔可感受到低音震动，具有强烈的仪式感染力。';
    else feeling = '声压级极高，万鼓齐鸣般震撼，展现古代祭祀大典的磅礴气势。';

    document.getElementById('ritual-feeling').textContent = feeling;
  }

  setupPlayMode() {
    const canvas = document.getElementById('play-drum-surface');
    if (!canvas) return;

    canvas.addEventListener('click', (e) => {
      const rect = canvas.getBoundingClientRect();
      const x = (e.clientX - rect.left) / rect.width;
      const y = (e.clientY - rect.top) / rect.height;
      this.handleDrumTap(x, y);
    });

    const forceSlider = document.getElementById('play-force');
    if (forceSlider) {
      forceSlider.addEventListener('input', () => {
        document.getElementById('play-force-val').textContent = forceSlider.value;
      });
    }

    const drumSelect = document.getElementById('play-drum-select');
    if (drumSelect) {
      drumSelect.addEventListener('change', () => {
        this.currentDrumId = drumSelect.value;
        const drum = this.ethnicLibrary.find(d => d.drum_id === drumSelect.value);
        if (drum) {
          document.getElementById('play-fund-freq').textContent = drum.fundamental_hz + 'Hz';
          document.getElementById('play-diam').textContent = drum.diameter_cm + 'cm';
        }
        this.drawPlayDrumSurface();
      });
      setTimeout(() => {
        this.currentDrumId = drumSelect.value;
        const drum = this.ethnicLibrary.find(d => d.drum_id === drumSelect.value);
        if (drum) {
          document.getElementById('play-fund-freq').textContent = drum.fundamental_hz + 'Hz';
          document.getElementById('play-diam').textContent = drum.diameter_cm + 'cm';
        }
        this.drawPlayDrumSurface();
      }, 100);
    }

    document.addEventListener('keydown', (e) => {
      const expView = document.getElementById('experience-view');
      const expPlay = document.getElementById('exp-play');
      if (!expView || expView.classList.contains('hidden')) return;
      if (!expPlay || expPlay.classList.contains('hidden')) return;

      const force = parseFloat(document.getElementById('play-force').value || '1');
      let x, y;

      switch (e.key.toLowerCase()) {
        case 'a':
          x = 0.5; y = 0.5;
          this.handleDrumTap(x, y, force * 0.7);
          break;
        case 's':
          x = 0.5; y = 0.35;
          this.handleDrumTap(x, y, force * 0.8);
          break;
        case 'd':
          x = 0.5; y = 0.2;
          this.handleDrumTap(x, y, force * 0.9);
          break;
        case ' ':
          e.preventDefault();
          x = 0.5; y = 0.5;
          this.handleDrumTap(x, y, force * 1.5);
          break;
      }
    });
  }

  async handleDrumTap(xFrac, yFrac, forceOverride) {
    const force = forceOverride || parseFloat(document.getElementById('play-force').value || '1');

    this.showTapRipple(xFrac, yFrac);

    let tapResult;
    try {
      tapResult = await this.apiPost('/api/experience/virtual-tap', {
        drum_id: this.currentDrumId,
        x_frac: xFrac,
        y_frac: yFrac,
        strike_force: force
      });
    } catch (e) {
      console.warn('Virtual tap API failed, using mock');
      tapResult = this.mockTapResult(xFrac, yFrac, force);
    }

    this.playDrumSound(tapResult);
    this.updatePlayDisplay(tapResult);
  }

  mockTapResult(xFrac, yFrac, force) {
    const drum = this.ethnicLibrary.find(d => d.drum_id === this.currentDrumId);
    const fund = drum ? drum.fundamental_hz : 150;

    const r = Math.sqrt((xFrac - 0.5) ** 2 + (yFrac - 0.5) ** 2) * 2;

    const harmonics = [];
    for (let n = 1; n <= 8; n++) {
      const ratio = n * 1.5 + 0.3;
      const edgeBoost = r * 0.8 + 0.2;
      const centerBoost = (1 - r) * 0.6 + 0.4;
      const weight = (n === 1 ? centerBoost : edgeBoost) / Math.sqrt(n);
      harmonics.push([fund * ratio, weight]);
    }

    const spectrum = [];
    for (let f = 20; f <= 3000; f += 5) {
      let amp = 0;
      harmonics.forEach(([freq, w]) => {
        const width = freq * 0.02 + 3;
        const diff = Math.abs(f - freq);
        if (diff < width * 3) {
          amp += Math.exp(-(diff * diff) / (2 * width * width)) * w;
        }
      });
      spectrum.push({ frequency_hz: f, amplitude_db: amp > 0 ? 20 * Math.log10(amp) + 80 : -60 });
    }

    const decay = 1.5 + (1 - r) * 1.5;
    const attack = 0.005 + r * 0.01;

    let zoneName, timbre;
    if (r < 0.3) { zoneName = '鼓心（太阳纹）'; timbre = '浑厚深沉，基音强，余韵悠长'; }
    else if (r < 0.6) { zoneName = '鼓面中部'; timbre = '平衡饱满，高中低频兼备'; }
    else if (r < 0.85) { zoneName = '鼓面边缘'; timbre = '明亮清脆，泛音丰富，穿透力强'; }
    else { zoneName = '鼓边'; timbre = '金属撞击声，短促刺耳'; }

    const envelope = [];
    for (let t = 0; t < 50; t++) {
      const time = t * 0.06;
      let env;
      if (time < attack) env = time / attack;
      else if (time < attack + decay * 0.3) env = 1 - (1 - 0.3) * (time - attack) / (decay * 0.3);
      else env = 0.3 * Math.exp(-(time - attack - decay * 0.3) / (decay * 0.7));
      envelope.push([time, env]);
    }

    return {
      spectrum,
      fundamental_freq_hz: fund,
      harmonics,
      amplitude_envelope: envelope,
      decay_time_s: decay,
      brightness: (r * 50 + 20),
      timbre_character: timbre,
      zone_name: zoneName,
      web_audio_params: {
        base_freq: fund,
        harmonic_gains: harmonics.map(h => h[1]),
        attack_s: attack,
        decay_s: decay * 0.3,
        sustain_level: 0.3,
        release_s: decay * 0.7,
        filter_cutoff_hz: 500 + r * 3000,
        filter_q: 1 + (1 - r) * 2,
        overall_gain: (0.3 + r * 0.7) * force
      }
    };
  }

  drawPlayDrumSurface() {
    const canvas = document.getElementById('play-drum-surface');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const size = Math.min(canvas.width, canvas.height);
    const cx = size / 2;
    const cy = size / 2;
    const radius = size * 0.45;

    ctx.clearRect(0, 0, size, size);

    const bgGrad = ctx.createRadialGradient(cx, cy, 0, cx, cy, radius);
    bgGrad.addColorStop(0, '#78350f');
    bgGrad.addColorStop(0.3, '#92400e');
    bgGrad.addColorStop(0.6, '#b45309');
    bgGrad.addColorStop(0.9, '#78350f');
    bgGrad.addColorStop(1, '#451a03');

    ctx.fillStyle = bgGrad;
    ctx.beginPath();
    ctx.arc(cx, cy, radius, 0, Math.PI * 2);
    ctx.fill();

    ctx.strokeStyle = '#fbbf24';
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.95, 0, Math.PI * 2);
    ctx.stroke();

    ctx.strokeStyle = '#f59e0b';
    ctx.lineWidth = 1.5;
    for (let i = 1; i <= 3; i++) {
      ctx.beginPath();
      ctx.arc(cx, cy, radius * (0.3 + i * 0.2), 0, Math.PI * 2);
      ctx.stroke();
    }

    const rays = 12;
    ctx.strokeStyle = '#fbbf24';
    ctx.lineWidth = 2;
    for (let i = 0; i < rays; i++) {
      const angle = (i / rays) * Math.PI * 2;
      const inner = radius * 0.1;
      const outer = radius * 0.5;
      ctx.beginPath();
      ctx.moveTo(cx + Math.cos(angle) * inner, cy + Math.sin(angle) * inner);
      ctx.lineTo(cx + Math.cos(angle) * outer, cy + Math.sin(angle) * outer);
      ctx.stroke();
    }

    ctx.fillStyle = '#fbbf24';
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.08, 0, Math.PI * 2);
    ctx.fill();

    const frogCount = 4;
    for (let i = 0; i < frogCount; i++) {
      const angle = (i / frogCount) * Math.PI * 2 + Math.PI / 4;
      const fx = cx + Math.cos(angle) * radius * 0.85;
      const fy = cy + Math.sin(angle) * radius * 0.85;
      ctx.fillStyle = '#78350f';
      ctx.beginPath();
      ctx.ellipse(fx, fy, 12, 8, angle, 0, Math.PI * 2);
      ctx.fill();
    }

    ctx.strokeStyle = 'rgba(251, 191, 36, 0.3)';
    ctx.lineWidth = 1;
    ctx.setLineDash([5, 5]);
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.3, 0, Math.PI * 2);
    ctx.stroke();
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.6, 0, Math.PI * 2);
    ctx.stroke();
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.85, 0, Math.PI * 2);
    ctx.stroke();
    ctx.setLineDash([]);
  }

  showTapRipple(xFrac, yFrac) {
    const canvas = document.getElementById('play-drum-surface');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const size = Math.min(canvas.width, canvas.height);
    const cx = size * xFrac;
    const cy = size * yFrac;

    let radius = 0;
    const maxRadius = size * 0.4;
    const startTime = performance.now();

    const animate = () => {
      const elapsed = performance.now() - startTime;
      const progress = Math.min(1, elapsed / 600);
      radius = maxRadius * progress;
      const alpha = 1 - progress;

      this.drawPlayDrumSurface();

      ctx.strokeStyle = `rgba(251, 191, 36, ${alpha * 0.8})`;
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.arc(cx, cy, radius, 0, Math.PI * 2);
      ctx.stroke();

      ctx.fillStyle = `rgba(251, 191, 36, ${alpha * 0.5})`;
      ctx.beginPath();
      ctx.arc(cx, cy, 8 * (1 - progress * 0.5), 0, Math.PI * 2);
      ctx.fill();

      if (progress < 1) {
        requestAnimationFrame(animate);
      } else {
        this.drawPlayDrumSurface();
      }
    };

    requestAnimationFrame(animate);
  }

  playDrumSound(tapResult) {
    if (!this.audioCtx) {
      try {
        this.audioCtx = new (window.AudioContext || window.webkitAudioContext)();
      } catch (e) {
        console.warn('Web Audio not supported');
        return;
      }
    }

    if (this.audioCtx.state === 'suspended') {
      this.audioCtx.resume();
    }

    const params = tapResult.web_audio_params;
    const now = this.audioCtx.currentTime;
    const masterGain = this.audioCtx.createGain();
    masterGain.gain.value = params.overall_gain * 0.15;

    const filter = this.audioCtx.createBiquadFilter();
    filter.type = 'lowpass';
    filter.frequency.value = params.filter_cutoff_hz;
    filter.Q.value = params.filter_q;

    params.harmonic_gains.forEach((gain, i) => {
      const freq = params.base_freq * (i + 1) * 1.5;
      const osc = this.audioCtx.createOscillator();
      osc.type = i === 0 ? 'sine' : (i < 3 ? 'triangle' : 'sine');
      osc.frequency.value = freq;

      const g = this.audioCtx.createGain();
      g.gain.setValueAtTime(0, now);
      g.gain.linearRampToValueAtTime(gain * 0.3, now + params.attack_s);
      g.gain.linearRampToValueAtTime(gain * 0.3 * params.sustain_level, now + params.attack_s + params.decay_s);
      g.gain.exponentialRampToValueAtTime(0.001, now + params.attack_s + params.decay_s + params.release_s + i * 0.05);

      osc.connect(g);
      g.connect(filter);
      osc.start(now);
      osc.stop(now + params.attack_s + params.decay_s + params.release_s + 1);
    });

    filter.connect(masterGain);
    masterGain.connect(this.audioCtx.destination);
  }

  updatePlayDisplay(tapResult) {
    document.getElementById('play-zone-info').textContent = tapResult.zone_name;
    document.getElementById('play-timbre').textContent = tapResult.timbre_character;
    document.getElementById('play-param-fund').textContent = tapResult.fundamental_freq_hz.toFixed(1) + 'Hz';
    document.getElementById('play-param-decay').textContent = tapResult.decay_time_s.toFixed(2) + 's';
    document.getElementById('play-param-bright').textContent = tapResult.brightness.toFixed(1);
    document.getElementById('play-param-attack').textContent = (tapResult.web_audio_params.attack_s * 1000).toFixed(0) + 'ms';

    this.drawPlaySpectrum(tapResult.spectrum);
    this.drawPlayEnvelope(tapResult.amplitude_envelope);
  }

  drawPlaySpectrum(spectrum) {
    const canvas = document.getElementById('play-spectrum');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    ctx.scale(dpr, dpr);

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    const maxF = 3000;
    const minAmp = -40;
    const maxAmp = 40;

    spectrum.forEach(bin => {
      const x = 30 + Math.max(0, Math.min(1, (bin.frequency_hz - 20) / (maxF - 20))) * (w - 40);
      const amp = Math.max(minAmp, Math.min(maxAmp, bin.amplitude_db));
      const barH = ((amp - minAmp) / (maxAmp - minAmp)) * (h - 30);
      const y = h - 15 - barH;

      const norm = (amp - minAmp) / (maxAmp - minAmp);
      const hue = 40 - norm * 40;
      ctx.fillStyle = `hsl(${hue}, 90%, 55%)`;
      ctx.fillRect(x - 1.5, y, 3, barH);
    });

    ctx.fillStyle = '#64748b';
    ctx.font = '9px sans-serif';
    ctx.textAlign = 'center';
    for (let f = 500; f <= maxF; f += 500) {
      const x = 30 + (f / maxF) * (w - 40);
      ctx.fillText(`${f}`, x, h - 5);
    }
  }

  drawPlayEnvelope(envelope) {
    const canvas = document.getElementById('play-envelope');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    ctx.scale(dpr, dpr);

    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    if (!envelope || envelope.length === 0) return;

    const maxAmp = Math.max(...envelope.map(e => e[1]));
    if (maxAmp === 0) return;

    ctx.beginPath();
    ctx.strokeStyle = '#f59e0b';
    ctx.lineWidth = 2;
    envelope.forEach(([t, a], i) => {
      const x = 20 + (t / 3) * (w - 30);
      const y = h - 15 - (a / maxAmp) * (h - 30);
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.stroke();

    ctx.fillStyle = 'rgba(245, 158, 11, 0.25)';
    ctx.beginPath();
    envelope.forEach(([t, a], i) => {
      const x = 20 + (t / 3) * (w - 30);
      const y = h - 15 - (a / maxAmp) * (h - 30);
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.lineTo(20 + (envelope[envelope.length - 1][0] / 3) * (w - 30), h - 15);
    ctx.lineTo(20, h - 15);
    ctx.closePath();
    ctx.fill();
  }

  drawCrossEraEmpty() { }
  drawRitualEmpty() { }
  drawPlayEmpty() { }
}
