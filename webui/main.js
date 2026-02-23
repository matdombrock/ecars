import init, { run_automaton } from '../pkg/ca.js';

async function main() {
  await init();
  const form = document.getElementById('params');
  const canvas = document.getElementById('output');
  const ctx = canvas.getContext('2d');

  form.onsubmit = async (e) => {
    e.preventDefault();
    const rule = parseInt(document.getElementById('rule').value, 10);
    const width = parseInt(document.getElementById('width').value, 10);
    const generations = parseInt(document.getElementById('generations').value, 10);
    const randomDistStr = document.getElementById('random_distribution').value;
    let random_distribution = null;
    if (randomDistStr !== '') {
      random_distribution = parseFloat(randomDistStr);
    }
    // Call wasm
    const result = run_automaton(rule, random_distribution, width, generations);
    // Draw result
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const cellW = Math.floor(canvas.width / width);
    const cellH = Math.floor(canvas.height / generations);
    for (let g = 0; g < generations; g++) {
      for (let x = 0; x < width; x++) {
        const idx = g * width + x;
        if (result[idx] === 1) {
          ctx.fillStyle = '#ff66cc';
          ctx.fillRect(x * cellW, g * cellH, cellW, cellH);
        }
      }
    }
  };
}

main();
