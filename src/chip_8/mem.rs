// 4 kB - 512 B
const DATA_SIZE: usize = 3584;

pub struct Memory {
    reg: Vec<u8>,
    mem: Vec<u8>,
    i: u8,
    timer: u8,
    sound: u8
}

impl Memory {
    // Initialize memory with all zeros. Memory skips the first 512 bytes to emulate space reserved
    // for interpreter. Since this is never touched, I removed it from accessible memory and just
    // started memory at 0
    pub fn new() -> Self {
        Self { reg: Vec::from([0; 16]), mem: Vec::new(), i: 0, timer: 0, sound: 0}
    }

    pub fn load_data(&mut self, data: Vec<u8>) ->  bool {

        if data.len() > DATA_SIZE {
            println!("Program size is too large!");
            return false;
        } else {
            self.mem = data.clone();
            return true;
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        return self.mem.clone();
    }
}
