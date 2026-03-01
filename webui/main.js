import init, { run_automaton, generate_automaton_image } from './pkg/ca.js';


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
    const colorIds = [
      'dead_color_from', 'dead_color_to', 'alive_color_from', 'alive_color_to'
    ];
    colorIds.forEach(id => {
      const el = document.getElementById(id);
      if (el) el.value = randomHexColor();
    });
    // Randomize shapes
    const shapeOptions = [
      'square', 'circle', 'circle-small', 'triangle-up', 'triangle-down',
      'triangle-left', 'triangle-right', 'triangle-r-a', 'triangle-r-b', 'triangle-r-c', 'triangle-r-d'
    ];
    const shapeIds = ['alive-shape', 'dead-shape'];
    shapeIds.forEach(id => {
      const el = document.getElementById(id);
      if (el) {
        const randomShape = shapeOptions[Math.floor(Math.random() * shapeOptions.length)];
        for (let i = 0; i < el.options.length; i++) {
          if (el.options[i].value === randomShape) {
            el.selectedIndex = i;
            break;
          }
        }
      }
    });
    // Randomize seed (64-bit unsigned integer)
    const randSeed = BigInt(Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)).toString();
    const seedEl = document.getElementById('seed');
    if (seedEl) seedEl.value = randSeed;
    // Trigger generation
    form.requestSubmit();
    form.dispatchEvent(new Event('submit', { cancelable: true }));
  };

  form.onsubmit = async (e) => {
    e.preventDefault();
    console.log('Generation Time');
    const genBtn = document.getElementById('generate-btn');
    genBtn.disabled = true;
    genBtn.textContent = 'Simulating';
    genBtn.classList.add('simulating');
    await new Promise(requestAnimationFrame); // Force browser repaint
    await new Promise(resolve => setTimeout(resolve, 1)); // Ensure "Generating..." is visible before heavy computation
    const rule = parseInt(document.getElementById('rule').value, 10);
    const width = parseInt(document.getElementById('width').value, 10);
    const generations = parseInt(document.getElementById('generations').value, 10);
    const randomDistStr = document.getElementById('random_distribution').value;
    let random_distribution = null;
    if (randomDistStr !== '') {
      random_distribution = parseFloat(randomDistStr);
    }
    const scale = parseInt(document.getElementById('scale').value, 10);
    const alive_shape = document.getElementById('alive-shape').value;
    const dead_shape = document.getElementById('dead-shape').value;
    const links = document.getElementById('links').checked;
    const mirror_x = document.getElementById('mirror_x').checked;
    const mirror_y = document.getElementById('mirror_y').checked;
    const dead_color_from = document.getElementById('dead_color_from').value;
    const dead_color_to = document.getElementById('dead_color_to').value;
    const alive_color_from = document.getElementById('alive_color_from').value;
    const alive_color_to = document.getElementById('alive_color_to').value;
    const seedStr = document.getElementById('seed').value;
    let seed = undefined;
    if (seedStr !== '') {
      try {
        seed = BigInt(seedStr);
      } catch {
        seed = undefined;
      }
    }

    // Call new WASM function to generate RGBA buffer
    const buffer = generate_automaton_image(
      rule,
      random_distribution,
      width,
      generations,
      seed,
      scale,
      alive_shape,
      dead_shape,
      links,
      dead_color_from,
      dead_color_to,
      alive_color_from,
      alive_color_to
    );

    // Always resize canvas to fit the full automaton
    const canvasWidth = width * scale;
    const canvasHeight = generations * scale;
    canvas.width = canvasWidth;
    canvas.height = canvasHeight;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Render RGBA buffer to canvas
    const imageData = new ImageData(new Uint8ClampedArray(buffer), canvasWidth, canvasHeight);
    ctx.putImageData(imageData, 0, 0);

    genBtn.disabled = false;
    genBtn.textContent = 'Generate';
    genBtn.classList.remove('simulating');
  };

}

// Ensure the DOM fully loads and attach event to the download button
window.addEventListener('DOMContentLoaded', () => {
  const downloadBtn = document.getElementById('download-image');
  const canvas = document.getElementById('output');
  if (downloadBtn && canvas) {
    downloadBtn.addEventListener('click', () => {
      const link = document.createElement('a');
      const timestamp = Date.now();
      link.download = `ecars-${timestamp}.png`;
      link.href = canvas.toDataURL();
      link.click();
    });
  }
});

main();
