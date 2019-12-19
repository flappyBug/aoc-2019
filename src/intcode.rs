use std::convert::TryInto;

pub struct Process {
    pub code: Vec<i32>,
    pub ip: usize,
    pub inputs: Vec<i32>,
    pub outputs: Vec<i32>,
}

impl Process {
    pub fn new(data: &str) -> Self {
        Self {
            code: data.split(',').map(|n| n.parse::<i32>().unwrap()).collect(),
            ip: 0,
            inputs: vec![],
            outputs: vec![],
        }
    }
    pub fn folk(&self) -> Self {
        Self {
            code: self.code.clone(),
            ip: self.ip,
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }

    pub fn pop_input(&mut self) -> i32 {
        self.inputs.pop().unwrap()
    }

    pub fn push_output(&mut self, value: i32) {
        self.outputs.push(value)
    }

    pub fn read<T: TryInto<usize>>(&self, addr: T) -> i32 {
        self.code[addr.try_into().ok().unwrap()]
    }

    pub fn indirect_read<T: TryInto<usize>>(&self, addr: T) -> i32 {
        self.read(self.read(addr))
    }

    pub fn write<T: TryInto<usize>, V: TryInto<i32>>(&mut self, addr: T, value: V) {
        self.code[addr.try_into().ok().unwrap()] = value.try_into().ok().unwrap()
    }

    pub fn indirect_write<T: TryInto<usize>, V: TryInto<i32>>(&mut self, addr: T, value: V) {
        self.write(self.read(addr), value)
    }

    pub fn is_finished(&self) -> bool {
        self.read(self.ip) == 99
    }

    pub fn step(&mut self) {
        let ip = self.ip;
        let op = self.read(ip);
        let opcode = op % 100;
        match opcode {
            1 | 2 | 7 | 8 => {
                let oprand1 = if op / 100 % 10 == 1 {
                    self.read(ip + 1)
                } else {
                    self.indirect_read(ip + 1)
                };
                let oprand2 = if op / 1000 % 10 == 1 {
                    self.read(ip + 2)
                } else {
                    self.indirect_read(ip + 2)
                };
                let value = match opcode {
                    1 => oprand1 + oprand2,
                    2 => oprand1 * oprand2,
                    7 => {
                        if oprand1 < oprand2 {
                            1
                        } else {
                            0
                        }
                    }
                    8 => {
                        if oprand1 == oprand2 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };
                self.indirect_write(ip + 3, value);
                self.ip += 4;
            }
            3 => {
                let input = self.pop_input();
                self.indirect_write(ip + 1, input);
                self.ip += 2;
            }
            4 => {
                let output = if op / 100 % 10 == 1 {
                    self.read(ip + 1)
                } else {
                    self.indirect_read(ip + 1)
                };
                self.push_output(output);
                self.ip += 2;
            }
            5 | 6 => {
                let cond = if op / 100 % 10 == 1 {
                    self.read(ip + 1)
                } else {
                    self.indirect_read(ip + 1)
                };
                if (opcode == 5 && cond != 0) || (opcode == 6 && cond == 0) {
                    let addr = if op / 1000 % 10 == 1 {
                        self.read(ip + 2)
                    } else {
                        self.indirect_read(ip + 2)
                    };
                    self.ip = addr as usize;
                } else {
                    self.ip += 3;
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn execute(&mut self) {
        while !self.is_finished() {
            self.step()
        }
    }
}
