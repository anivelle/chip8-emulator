mod mem;
// mod display;
// mod interpreter;

use mem::Memory;
// use display::Display;
// use interpreter::Interpreter;

pub struct Chip8 {
    pub memory: Memory,
}


impl Chip8 {
    pub fn new() -> Self {
        Self { memory: Memory::new(), }
    }

}
