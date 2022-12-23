mod chip_8;

use cairo;
use chip_8::Chip8;

// TODO: Implement structure
// Chip8
//   ├─────────────────┬─────────────────┐
//   ▼                 ▼                 │
// Memory           Display              │
//   ├─────────┐       ├───────┐         │
//   ▼         ▼       ▼       ▼         ▼
// Registers  Data  Sprites  Screen Interpreter
//      ▲      │                  ▲   ▲  ▲ ▲
//      │      │                  └───┘  │ │
//      │      └─────────────────────────┘ │
//      └──────────────────────────────────┘
//

fn main() {
    let mut chip8 = Chip8::new();
    let data: Vec<u8> = Vec::from([2; 16]);
    chip8.memory.load_data(data);

    let mut x = 0;
    for i in chip8.memory.get_data() {
        println!("{}: {:x}", x, i);
        x += 1;
    }
}
