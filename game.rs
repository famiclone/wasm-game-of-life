#![no_main]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

struct Rand {
    lo: u64,
    hi: u64,
}

impl Rand {
    const fn new(seed: u64) -> Self {
        let i = Rand {
            lo: seed,
            hi: seed + 1,
        };
        //Self::random(&mut i);
        i
    }

    // https://v8.dev/blog/math-random
    fn random(&mut self) -> usize {
        self.lo = 18030 * (self.lo & 0xFFFF) + (self.lo >> 16);
        self.hi = 30903 * (self.hi & 0xFFFF) + (self.hi >> 16);

        ((self.lo << 16) + self.hi) as usize / 10000000
        //((self.lo << 16) + self.hi) as f64 / 100000000000000.0 as f64 // 10_f64.powi(14) 14 - number of digits TODO: calc
    }
}

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

type Color = [u8; 4];

pub struct Board {
    data: [Color; WIDTH * HEIGHT],
}

impl Board {
    fn fill(&mut self, value: Color) {
        for p in self.data.iter_mut() {
            *p = value;
        }
    }

    fn draw(&mut self, value: Color, i: usize) {
        self.data[i] = value;
    }
}

struct Game {
    rand: Rand,
}

impl Game {
    const fn new() -> Self {
        Self {
            rand: Rand::new(1234567890),
        }
    }

    fn update(&mut self, ts: usize) {}

    fn render(&mut self, board: &mut Board) {
        for p in 0..WIDTH * HEIGHT {
            let mut max: usize = 300;
            let r = self.rand.random();

            if self.rand.random() > max {
                board.draw([0, 0, 0, 255], p);
                //board[p] = [0, 0, 0, 255];
            } else {
                board.draw([255, 255, 255, 255], p);
                //board[p] = [255, 255, 255, 255];
            }
        }
    }
}

static mut GAME: Game = Game::new();
static mut BOARD: Board = Board {
    data: [[0, 0, 0, 255]; WIDTH * HEIGHT],
};

#[no_mangle]
pub extern "C" fn get_width() -> usize {
    WIDTH
}

#[no_mangle]
pub extern "C" fn get_height() -> usize {
    HEIGHT
}

#[no_mangle]
pub unsafe extern "C" fn update(ts: usize) {
    GAME.update(ts);
    GAME.render(&mut BOARD);
}

#[no_mangle]
pub unsafe extern "C" fn get_board() -> &'static mut Board {
    &mut BOARD
}
