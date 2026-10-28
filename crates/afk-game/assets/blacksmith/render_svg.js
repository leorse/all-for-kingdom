// Rend un SVG en PNG transparent avec Chromium (via Playwright), à l'échelle voulue.
// Usage : node render_svg.js entrée.svg sortie.png [échelle=2] [fond=transparent|#rrggbb]
const fs = require('fs');
const { chromium } = require('playwright');

const [input, output, scaleArg, bg] = process.argv.slice(2);
if (!input || !output) {
  console.error('usage: node render_svg.js entrée.svg sortie.png [échelle] [fond]');
  process.exit(1);
}
const scale = Number(scaleArg || 2);
const svg = fs.readFileSync(input, 'utf8');
const m = svg.match(/width="(\d+)"\s+height="(\d+)"/);
const [w, h] = [Number(m[1]), Number(m[2])];

(async () => {
  const browser = await chromium.launch({
    executablePath: process.env.CHROMIUM_PATH || undefined,
  });
  const page = await browser.newPage({ deviceScaleFactor: scale, viewport: { width: w, height: h } });
  const background = bg && bg !== 'transparent' ? bg : 'transparent';
  await page.setContent(`<body style="margin:0;background:${background}">${svg}</body>`);
  await page.screenshot({ path: output, omitBackground: background === 'transparent' });
  await browser.close();
  console.log(`écrit ${output} (${w * scale}x${h * scale})`);
})();
