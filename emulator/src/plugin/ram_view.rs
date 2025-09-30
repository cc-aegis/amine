use minifb::{Scale, ScaleMode, Window, WindowOptions};
use crate::cpu::CPU;
use crate::plugin::Plugin;

pub struct RamView {
    buffer: Box<[u32; 256 * 256]>,
    window: Window,
}

impl RamView {
    pub fn new() -> Result<Self, minifb::Error> {
        let buffer = Box::new([0; 256 * 256]);
        let window = Window::new(
            "RAM View",
            256,
            256,
            WindowOptions {
                resize: true,
                scale: Scale::X2,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..Default::default()
            },
        )?;
        Ok(RamView { buffer, window })
    }
}

impl Plugin for RamView {
    fn update(&mut self, cpu: &mut CPU) {
        for (idx, val) in cpu.ram.iter().enumerate() {
            //let c = (*val >> 8) as u32;
            //let rgb = (c << 24) | (c << 16) | (c << 8);
            let blue = if *val == 0 { 0 } else { 0x80 };
            let rgb = 0xFF000000 | ((*val as u32) << 8) | blue;
            self.buffer[idx] = rgb;
        }

        if self.window.is_open() {
            self.window
                .update_with_buffer(self.buffer.as_slice(), 256, 256)
                .unwrap();
        }
    }
}