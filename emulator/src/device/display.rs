use minifb::{Scale, ScaleMode, Window, WindowOptions};
use crate::device::Device;

const WIDTH: usize = 128;
const HEIGHT: usize = 96;

pub struct Display {
    buffer: Box<[u32; WIDTH * HEIGHT]>,
    window: Window,
    cmd: u16,
    x: u16,
    y: u16,
    color: u32,
}

impl Display {
    pub fn new() -> Self {
        Display {
            buffer: Box::new([0; WIDTH * HEIGHT]),
            window: Window::new(
                "8 Bit Display",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: true,
                    scale: Scale::X4,
                    scale_mode: ScaleMode::AspectRatioStretch,
                    ..Default::default()
                },
            ).unwrap(),
            cmd: 0,
            x: 0,
            y: 0,
            color: 0,
        }
    }
}

fn unpack_color(color: u8) -> u32 {
    let w = color as u32 & 0x03;

    let r = (color as u32 & 0xC0) << 16 | w << 20;
    let g = (color as u32 & 0x30) << 10 | w << 12;
    let b = (color as u32 & 0x0C) << 4 | w << 4;

    0xFF000000 | r | g | b
}

impl Device for Display {
    fn set_context(&mut self, idx: u16, value: u16) {
        match idx {
            0 => self.cmd = value,
            1 => self.x = value,
            2 => self.y = value,
            3 => self.color = unpack_color(value as u8),
            _ => {},
        }
    }

    fn send(&mut self) {
        match self.cmd {
            1 => if self.x < 128 && self.y < 96 {
                //println!("x: {}, y: {}, c: {}", self.x, self.y, self.color);
                self.buffer[self.x as usize + self.y as usize * WIDTH] = self.color;
            },
            2 => if self.window.is_open() {
                self.window
                    .update_with_buffer(self.buffer.as_slice(), WIDTH, HEIGHT)
                    .unwrap();
            },
            _ => {},
        }
    }
}