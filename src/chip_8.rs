mod inst;
mod mem;

use cairo::*;
use inst::Instructions;
use mem::*;

pub struct Chip8 {
    pub memory: Memory,
    pub disp: ImageSurface,
    pub instructions: Instructions,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            disp: ImageSurface::create(Format::A8, 64, 32).unwrap(),
            instructions: Instructions::new()
        }
    }

    pub fn set_display(&mut self) {

    }
}
