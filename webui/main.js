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
    console.log('Generation Time');
    const generatingEl = document.getElementById('generating');
    generatingEl.style.display = 'block';
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
    const circles = document.getElementById('circles').checked;
    const links = document.getElementById('links').checked;
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

    // Call new WASM function to generate RGBA buffer
    const buffer = generate_automaton_image(
      rule,
      random_distribution,
      width,
      generations,
      seed,
      scale,
      circles,
      links,
      bg_from,
      bg_to,
      fg_from,
      fg_to
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

    console.log('Generation over');
    generatingEl.style.display = 'none';
  };

}

// Ensure the DOM fully loads and attach event to the download button
window.addEventListener('DOMContentLoaded', () => {
  const downloadBtn = document.getElementById('download-image');
  const canvas = document.getElementById('output');
  if (downloadBtn && canvas) {
    downloadBtn.addEventListener('click', () => {
      const link = document.createElement('a');
      link.download = 'automaton.png';
      link.href = canvas.toDataURL();
      link.click();
    });
  }
});

main();
