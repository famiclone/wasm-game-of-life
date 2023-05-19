const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

async function run() {
  const { instance: { exports } } = await WebAssembly.instantiateStreaming(fetch('game.wasm'));

  const memoryView = new Uint8Array(exports.memory.buffer);

  console.log(memoryView);

  const SCALE = 50;
  const WIDTH = exports.get_width();
  const HEIGHT = exports.get_height();

  canvas.width = WIDTH * SCALE;
  canvas.height = HEIGHT * SCALE;

  let now = 0;

  function render(data) {
    const image = new ImageData(data, WIDTH, HEIGHT);

    ctx.clearRect(0, 0, WIDTH, HEIGHT);

    ctx.putImageData(image, 0, 0);
    ctx.scale(SCALE, SCALE);
  }

  const board = exports.get_board();


  function loop(ts) {
    const dt = ts - now;
    now = ts;

    exports.update(dt);

    render(new Uint8ClampedArray(memoryView.subarray(board, board + 4 * (WIDTH * HEIGHT))));

    requestAnimationFrame(loop);
  }

  requestAnimationFrame(loop);
}

run().catch(console.error);
