export function setupHiDPICanvas(canvasId, width, height) {
    const canvas = document.getElementById(canvasId);
    if (!canvas) return null;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = width * dpr;
    canvas.height = height * dpr;
    canvas.style.width = width + 'px';
    canvas.style.height = height + 'px';
    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);
    return { canvas, ctx, width, height };
}

export function drawMultiLineChart(ctx, width, height, series, opts = {}) {
    const {
        xLabel = '', yLabel = '', title = '',
        colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899'],
        yLog = false, padding = { top: 30, right: 20, bottom: 45, left: 55 },
    } = opts;

    ctx.clearRect(0, 0, width, height);

    let allX = [], allY = [];
    for (const s of series) {
        for (const [x, y] of s.data) { allX.push(x); allY.push(y); }
    }
    if (!allX.length) return;
    const xMin = Math.min(...allX), xMax = Math.max(...allX);
    let yMin = Math.min(...allY.filter(v => v > 0)), yMax = Math.max(...allY);
    if (!yMin || !isFinite(yMin)) yMin = 0.1;
    if (!yMax || !isFinite(yMax)) yMax = 1;

    const plotW = width - padding.left - padding.right;
    const plotH = height - padding.top - padding.bottom;

    const mapX = x => padding.left + (x - xMin) / (xMax - xMin + 1e-9) * plotW;
    let mapY;
    if (yLog) {
        const lyMin = Math.log10(yMin), lyMax = Math.log10(yMax);
        mapY = y => padding.top + plotH - (Math.log10(Math.max(y, yMin)) - lyMin) / (lyMax - lyMin + 1e-9) * plotH;
    } else {
        mapY = y => padding.top + plotH - (y - yMin) / (yMax - yMin + 1e-9) * plotH;
    }

    ctx.strokeStyle = 'rgba(148,163,184,0.25)';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 5; i++) {
        const y = padding.top + plotH * i / 5;
        ctx.beginPath(); ctx.moveTo(padding.left, y); ctx.lineTo(width - padding.right, y); ctx.stroke();
    }

    ctx.fillStyle = '#94a3b8'; ctx.font = '11px sans-serif'; ctx.textAlign = 'right';
    for (let i = 0; i <= 5; i++) {
        const frac = 1 - i / 5;
        const yv = yLog ? Math.pow(10, Math.log10(yMin) + (Math.log10(yMax) - Math.log10(yMin)) * frac)
            : yMin + (yMax - yMin) * frac;
        ctx.fillText(yv.toFixed(yv < 10 ? 2 : 0), padding.left - 6, padding.top + plotH * i / 5 + 4);
    }
    ctx.textAlign = 'center';
    for (let i = 0; i <= 5; i++) {
        const xv = xMin + (xMax - xMin) * i / 5;
        ctx.fillText(xv.toFixed(xv < 10 ? 1 : 0), padding.left + plotW * i / 5, height - padding.bottom + 16);
    }

    series.forEach((s, idx) => {
        const color = colors[idx % colors.length];
        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.beginPath();
        s.data.forEach(([x, y], i) => {
            const px = mapX(x), py = mapY(y);
            if (i === 0) ctx.moveTo(px, py); else ctx.lineTo(px, py);
        });
        ctx.stroke();
    });

    if (title) {
        ctx.fillStyle = '#e2e8f0'; ctx.font = 'bold 13px sans-serif'; ctx.textAlign = 'center';
        ctx.fillText(title, width / 2, 18);
    }
    if (xLabel) {
        ctx.fillStyle = '#94a3b8'; ctx.font = '11px sans-serif';
        ctx.fillText(xLabel, width / 2, height - 8);
    }
    if (yLabel) {
        ctx.save();
        ctx.translate(14, height / 2);
        ctx.rotate(-Math.PI / 2);
        ctx.fillText(yLabel, 0, 0);
        ctx.restore();
    }

    if (series.length && series[0].name) {
        let ly = padding.top + 4;
        ctx.font = '11px sans-serif';
        series.forEach((s, idx) => {
            ctx.fillStyle = colors[idx % colors.length];
            ctx.fillRect(width - padding.right - 120, ly, 10, 10);
            ctx.fillStyle = '#e2e8f0'; ctx.textAlign = 'left';
            ctx.fillText(s.name || `Series ${idx + 1}`, width - padding.right - 105, ly + 9);
            ly += 14;
        });
    }
}

export function drawHeatmap(ctx, width, height, data, opts = {}) {
    const {
        xRange = [-1, 1], yRange = [-1, 1],
        valueLabel = 'SPL (dB)', title = '',
        padding = { top: 30, right: 20, bottom: 45, left: 55 },
    } = opts;

    ctx.clearRect(0, 0, width, height);
    if (!data.length) return;

    const [xMin, xMax] = xRange, [yMin, yMax] = yRange;
    const values = data.map(d => d.val);
    const vMin = Math.min(...values), vMax = Math.max(...values);

    const plotW = width - padding.left - padding.right;
    const plotH = height - padding.top - padding.bottom;

    const xs = [...new Set(data.map(d => d.x))].sort((a, b) => a - b);
    const ys = [...new Set(data.map(d => d.y))].sort((a, b) => a - b);
    if (xs.length < 2 || ys.length < 2) return;

    const cellW = plotW / (xs.length - 1);
    const cellH = plotH / (ys.length - 1);

    const valMap = new Map(data.map(d => [`${d.x.toFixed(5)}|${d.y.toFixed(5)}`, d.val]));
    for (let ix = 0; ix < xs.length; ix++) {
        for (let iy = 0; iy < ys.length; iy++) {
            const key = `${xs[ix].toFixed(5)}|${ys[iy].toFixed(5)}`;
            const val = valMap.get(key);
            if (val === undefined) continue;
            const frac = vMax === vMin ? 0 : (val - vMin) / (vMax - vMin);
            const r = Math.round(50 + frac * 200);
            const g = Math.round(80 + frac * 100);
            const b = Math.round(200 - frac * 150);
            ctx.fillStyle = `rgba(${r},${g},${b},0.85)`;
            const px = padding.left + (xs[ix] - xMin) / (xMax - xMin + 1e-9) * plotW - cellW / 2;
            const py = padding.top + plotH - (ys[iy] - yMin) / (yMax - yMin + 1e-9) * plotH - cellH / 2;
            ctx.fillRect(px, py, cellW + 1, cellH + 1);
        }
    }

    ctx.fillStyle = '#94a3b8'; ctx.font = '11px sans-serif'; ctx.textAlign = 'center';
    for (let i = 0; i <= 5; i++) {
        const xv = xMin + (xMax - xMin) * i / 5;
        ctx.fillText(xv.toFixed(1), padding.left + plotW * i / 5, height - padding.bottom + 16);
    }
    ctx.textAlign = 'right';
    for (let i = 0; i <= 5; i++) {
        const yv = yMin + (yMax - yMin) * i / 5;
        ctx.fillText(yv.toFixed(1), padding.left - 6, padding.top + plotH - plotH * i / 5 + 4);
    }

    const barX = width - padding.right + 6, barY = padding.top, barW = 10, barH = plotH;
    for (let i = 0; i < barH; i++) {
        const frac = 1 - i / barH;
        const r = Math.round(50 + frac * 200), g = Math.round(80 + frac * 100), b = Math.round(200 - frac * 150);
        ctx.fillStyle = `rgb(${r},${g},${b})`;
        ctx.fillRect(barX, barY + i, barW, 1);
    }
    ctx.fillStyle = '#94a3b8'; ctx.textAlign = 'left'; ctx.font = '10px sans-serif';
    ctx.fillText(vMax.toFixed(0), barX + barW + 4, barY + 4);
    ctx.fillText(vMin.toFixed(0), barX + barW + 4, barY + barH);
    ctx.fillText(valueLabel, barX - 20, barY - 6);

    if (title) {
        ctx.fillStyle = '#e2e8f0'; ctx.font = 'bold 13px sans-serif'; ctx.textAlign = 'center';
        ctx.fillText(title, width / 2, 18);
    }
}

export function drawBarChart(ctx, width, height, bars, opts = {}) {
    const {
        title = '', colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444'],
        valueSuffix = '',
        padding = { top: 30, right: 20, bottom: 50, left: 55 },
    } = opts;
    ctx.clearRect(0, 0, width, height);
    if (!bars.length) return;

    const plotW = width - padding.left - padding.right;
    const plotH = height - padding.top - padding.bottom;
    const maxV = Math.max(...bars.map(b => b.value), 0.1);
    const bw = plotW / bars.length * 0.7;
    const gap = plotW / bars.length * 0.3;

    bars.forEach((b, i) => {
        const bh = (b.value / maxV) * plotH;
        const x = padding.left + i * (bw + gap) + gap / 2;
        const y = padding.top + plotH - bh;
        const g = ctx.createLinearGradient(0, y, 0, y + bh);
        g.addColorStop(0, colors[i % colors.length]);
        g.addColorStop(1, '#1e293b');
        ctx.fillStyle = g;
        ctx.fillRect(x, y, bw, bh);

        ctx.fillStyle = '#e2e8f0'; ctx.font = '11px sans-serif'; ctx.textAlign = 'center';
        ctx.fillText(b.label.slice(0, 8), x + bw / 2, height - padding.bottom + 16);
        ctx.fillText(b.value.toFixed(1) + valueSuffix, x + bw / 2, y - 5);
    });

    if (title) {
        ctx.fillStyle = '#e2e8f0'; ctx.font = 'bold 13px sans-serif'; ctx.textAlign = 'center';
        ctx.fillText(title, width / 2, 18);
    }
}
