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
  ctx.scale(SCALE, SCALE);

  let now = 0;

  function render(data) {
    const image = new ImageData(data, WIDTH, HEIGHT);

    console.log(image);
    ctx.clearRect(0, 0, WIDTH, HEIGHT);

    ctx.putImageData(image, 0, 0);
  }

    const board = exports.get_board();

    render(new Uint8ClampedArray(memoryView.subarray(board, board + 4 * (WIDTH * HEIGHT))));

  function loop(ts) {
    const dt = ts - now;
    now = ts;

    exports.update(dt);


    requestAnimationFrame(loop);
  }

  requestAnimationFrame(loop);
}

run().catch(console.error);
