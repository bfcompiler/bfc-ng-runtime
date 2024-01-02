#[used]
#[link_section = ".bfc"]
static VAR_BFBM: u32 = 30_000; // 30k cells, around 29.29kb of possible memory
#[used]
#[link_section = ".bfc"]
static VAR_BFB: &[u8] = &VAR_BFBR;
#[used]
#[link_section = ".bfc"]
static VAR_BFBR: [u8; 0x20000] = [0x00; 0x20000]; // 128kb of possible instruction storage, or 262,144 instructions

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    EndExecution,
    ErrorValue,
}

#[derive(Debug, Clone)]
struct ExecutionManager {
    pub cells: Vec<u8>,
    pub iptr: u32,
    pub rptr: u32,
    pub mptr: u32,
    pub cins: Vec<Instruction>,
    pub loops: Vec<u32>,
    pub end: bool,
}

impl ExecutionManager {
    pub fn new() -> Self {
        Self {
            cells: vec![0x00u8; VAR_BFBM as usize],
            iptr: 0,
            rptr: 0,
            mptr: 0,
            cins: vec![],
            loops: vec![],
            end: false,
        }
    }

    pub fn cache_instructions(&mut self, count: u32) {
        let count = (count as f32 / 2.0).ceil() as u32;
        for _ in 0..count {
            // let (insr, insl) = Instruction::parse_byte(VAR_BFBR[self.rptr as usize]);
            let (insr, insl) = Instruction::parse_byte(unsafe {
                std::ptr::read_volatile(std::ptr::addr_of!(VAR_BFBR).cast::<u8>().offset(self.rptr as isize))
            });
            self.rptr += 1;
            self.cins.push(insr);
            self.cins.push(insl);
        }
    }

    pub fn execute_instructions(&mut self) {
        use std::io::Write;
        // self.trim_end_executions();
        let mut index: u32 = 0;
        loop {
            if self.iptr == self.cins.len() as u32 {
                break;
            }
            let ins = &self.cins[self.iptr as usize];
            match ins {
                Instruction::ShiftRight => {
                    if self.mptr == VAR_BFBM - 1 {
                        self.mptr = 0;
                    } else {
                        self.mptr += 1;
                    }
                }
                Instruction::ShiftLeft => {
                    if self.mptr == 0 {
                        self.mptr = VAR_BFBM - 1;
                    } else {
                        self.mptr -= 1;
                    }
                }
                Instruction::Increment => {
                    if self.cells[self.mptr as usize] == 255 {
                        self.cells[self.mptr as usize] = 0;
                    } else {
                        self.cells[self.mptr as usize] += 1;
                    }
                }
                Instruction::Decrement => {
                    if self.cells[self.mptr as usize] == 0 {
                        self.cells[self.mptr as usize] = 255;
                    } else {
                        self.cells[self.mptr as usize] -= 1;
                    }
                }
                Instruction::Output => {
                    print!("{}", self.cells[self.mptr as usize] as char);
                }
                Instruction::LoopStart => {
                    self.loops.push(self.iptr);
                }
                Instruction::LoopEnd => {
                    if self.cells[self.mptr as usize] == 0 {
                        self.loops.pop();
                    } else {
                        self.iptr = self.loops[self.loops.len() - 1] as u32;
                    }
                }
                Instruction::EndExecution => {
                    self.end = true;
                    return;
                }
                _ => {}
            }
            self.iptr += 1;
        }
    }

    pub fn poll(&self) -> bool {
        return !self.end;
    }

    pub fn trim_end_executions(&mut self) {
        let dlen = self.cins.len();
        let mut found = false;
        for index in 0..dlen {
            let index = dlen - index - 1;
            let ins = self.cins.get(index).unwrap();
            if ins == &Instruction::EndExecution {
                self.cins.pop();
                found = true;
            } else {
                break;
            }
        }
        if found {
            self.cins.push(Instruction::EndExecution);
        }
    }
}

impl Instruction {
    pub fn parse_byte(byte: u8) -> (Self, Self) {
        let insc = (byte & 0xC0) >> 6;
        match insc {
            0x03 => {
                // Both instructions
                let insr = byte & 0x07;
                let insl = (byte & 0x38) >> 3;
                return (
                    Self::parse_single_from_byte(insr),
                    Self::parse_single_from_byte(insl),
                );
            }
            0x01 => {
                // Right instruction then end execution
                let insr = byte & 0x07;
                return (Self::parse_single_from_byte(insr), Self::EndExecution);
            }
            _ => {
                // End execution now
                return (Self::EndExecution, Self::EndExecution);
            }
        }
    }

    pub fn parse_single_from_byte(val: u8) -> Self {
        match val {
            0x00 => return Self::ShiftRight,
            0x01 => {
                return Self::ShiftLeft;
            }
            0x02 => {
                return Self::Increment;
            }
            0x03 => {
                return Self::Decrement;
            }
            0x04 => {
                return Self::Output;
            }
            0x05 => {
                return Self::Input;
            }
            0x06 => {
                return Self::LoopStart;
            }
            0x07 => {
                return Self::LoopEnd;
            }
            _ => {
                return Self::ErrorValue;
            }
        }
    }
}

fn main() {
    let mut manager = ExecutionManager::new();
    manager.cache_instructions(262_144);
    manager.execute_instructions();
}
