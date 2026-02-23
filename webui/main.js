import init, { run_automaton } from '../pkg/ca.js';

async function main() {
  await init();
  const form = document.getElementById('params');
  const canvas = document.getElementById('output');
  const ctx = canvas.getContext('2d');

  // Randomize button logic
  document.getElementById('randomize').onclick = () => {
    // Randomize rule
    document.getElementById('rule').value = Math.floor(Math.random() * 256);
    // Randomize random distribution
    document.getElementById('random_distribution').value = (Math.random()).toFixed(2);
    // Randomize colors
    function randomHexColor() {
      return '#' + Math.floor(Math.random() * 0xffffff).toString(16).padStart(6, '0');
    }
    document.getElementById('bg_from').value = randomHexColor();
    document.getElementById('bg_to').value = randomHexColor();
    document.getElementById('fg_from').value = randomHexColor();
    document.getElementById('fg_to').value = randomHexColor();
    // Randomize seed (64-bit unsigned integer)
    const randSeed = BigInt(Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)).toString();
    document.getElementById('seed').value = randSeed;
    // Trigger generation
    form.requestSubmit();
  };


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
    const scale = parseInt(document.getElementById('scale').value, 10);
    const circles = document.getElementById('circles').checked;
    const bg_from = document.getElementById('bg_from').value;
    const bg_to = document.getElementById('bg_to').value;
    const fg_from = document.getElementById('fg_from').value;
    const fg_to = document.getElementById('fg_to').value;
    const seedStr = document.getElementById('seed').value;
    let seed = undefined;
    if (seedStr !== '') {
      try {
        seed = BigInt(seedStr);
      } catch {
        seed = undefined;
      }
    }

    // Call wasm
    const result = run_automaton(rule, random_distribution, width, generations, seed);



    // Always resize canvas to fit the full automaton
    const canvasWidth = width * scale;
    const canvasHeight = generations * scale;
    canvas.width = canvasWidth;
    canvas.height = canvasHeight;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Color interpolation helpers
    function hexToRgb(hex) {
      hex = hex.replace('#', '');
      return [parseInt(hex.slice(0, 2), 16), parseInt(hex.slice(2, 4), 16), parseInt(hex.slice(4, 6), 16)];
    }
    function lerp(a, b, t) {
      return a + (b - a) * t;
    }
    function lerpColor(rgbA, rgbB, t) {
      return [
        Math.round(lerp(rgbA[0], rgbB[0], t)),
        Math.round(lerp(rgbA[1], rgbB[1], t)),
        Math.round(lerp(rgbA[2], rgbB[2], t)),
      ];
    }
    const bgA = hexToRgb(bg_from);
    const bgB = hexToRgb(bg_to);
    const fgA = hexToRgb(fg_from);
    const fgB = hexToRgb(fg_to);

    for (let g = 0; g < generations; g++) {
      const bgColor = lerpColor(bgA, bgB, generations > 1 ? g / (generations - 1) : 0);
      const fgColor = lerpColor(fgA, fgB, generations > 1 ? g / (generations - 1) : 0);
      for (let x = 0; x < width; x++) {
        const idx = g * width + x;
        if (result[idx] === 1) {
          ctx.fillStyle = `rgb(${fgColor[0]},${fgColor[1]},${fgColor[2]})`;
        } else {
          ctx.fillStyle = `rgb(${bgColor[0]},${bgColor[1]},${bgColor[2]})`;
        }
        if (circles) {
          ctx.beginPath();
          ctx.arc(x * scale + scale / 2, g * scale + scale / 2, scale / 2, 0, 2 * Math.PI);
          ctx.fill();
        } else {
          ctx.fillRect(x * scale, g * scale, scale, scale);
        }
      }
    }
  };
}

main();


