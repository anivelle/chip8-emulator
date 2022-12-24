// 4 kB - 512 B
const DATA_SIZE: usize = 3584;

pub struct Memory {
    reg: Vec<u8>,
    mem: Vec<u8>,
    digits: [[u8; 5]; 15],
    i: u8,
    timer: u8,
    sound: u8,
}

pub const DIGITS: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80]
];

impl Memory {
    // Initialize memory with all zeros. Memory skips the first 512 bytes to emulate space reserved
    // for interpreter. Since this is never touched, I removed it from accessible memory and just
    // started memory at 0
    pub fn new() -> Self {
        Self {
            reg: Vec::from([0; 16]),
            mem: Vec::new(),
            i: 0,
            timer: 0,
            sound: 0,
        }
    }

    /// Sets memory to a set of data
    /// Returns **true** if the data is set, **false**
    /// if the data is too long (no other checks)
    pub fn load_data(&mut self, data: Vec<u8>) -> bool {
        if data.len() > DATA_SIZE {
            println!("Program size is too large!");
            return false;
        } else {
            self.mem = data.clone();
            return true;
        }
    }

    /// Get the current state of Memory
    /// Returns a **Vec** of bytes
    pub fn get_data(&self) -> Vec<u8> {
        return self.mem.clone();
    }

    /// Set a register with an 8 bit value
    /// **name:** a value from 0-15, if the value is larger
    /// than this no registers are set
    pub fn set_reg(&mut self, name: usize, val: &u8) {
        if name <= 15 {
            self.reg[name] = *val;
        } else {
            println!("Error: entered invalid register");
        }
    }

    /// Get a register value
    /// **name:** a value from 015, if the value is larger
    /// than this **None** is returned
    pub fn get_reg(&self, name: usize) -> Option<u8> {
        if name <= 15 {
            Some(self.reg[name])
        } else {
            println!("Error: entered invalid register");
            None
        }
    }
}
