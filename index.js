const canvas = document.getElementById('canvas');
canvas.style.zoom = '1.8';

const ctx = canvas.getContext('2d');


async function run() {
  const { instance: { exports } } = await WebAssembly.instantiateStreaming(fetch('game.wasm'));
  let counter = 0;

  const memoryView = new Uint8Array(exports.memory.buffer);

  const SCALE = 1;
  const WIDTH = exports.get_width();
  const HEIGHT = exports.get_height();

  canvas.width = WIDTH * SCALE;
  canvas.height = HEIGHT * SCALE;

  let now = 0;

  function render(data) {
    ctx.clearRect(0, 0, WIDTH, HEIGHT);
    ctx.putImageData(new ImageData(data, WIDTH, HEIGHT), 0, 0);
  }

  const board = exports.get_board();


  function loop(ts) {
    const dt = ts - now;
    now = ts;

    counter += dt;

    if (counter > 1000) {
      counter = 0;
      exports.update(dt);
    }

    render(new Uint8ClampedArray(memoryView.subarray(board, board + 4 * (WIDTH * HEIGHT))));

    requestAnimationFrame(loop);
  }

  requestAnimationFrame(loop);
}

run().catch(console.error);
