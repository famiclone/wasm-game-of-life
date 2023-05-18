#![no_main]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

type Color = [u8; 4];

#[repr(C)]
pub struct Board {
    data: [Color; WIDTH * HEIGHT],
}

impl Board {
    fn fill(&mut self, value: Color) {
        for p in self.data.iter_mut() {
            *p = value;
        }
    }

    fn draw(&mut self, value: Color, x: usize, y: usize) {
        self.data[y * WIDTH + x] = value;
    }
}

static mut board: Board = Board {
    data: [[255, 0, 0, 255]; WIDTH * HEIGHT],
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
pub unsafe extern "C" fn get_board() -> &'static mut Board {
    &mut board
}

#[no_mangle]
pub extern "C" fn update(ts: usize) -> usize {
    ts
}
