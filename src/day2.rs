pub struct Process {
    code: Vec<u32>,
    ip: usize,
}

impl Process {
    pub fn folk(&self) -> Self {
        Self {
            code: self.code.clone(),
            ip: self.ip,
        }
    }

    pub fn read(&self, addr: usize) -> u32 {
        self.code[addr]
    }

    pub fn write(&mut self, addr: usize, value: u32) {
        self.code[addr] = value
    }

    pub fn is_finished(&self) -> bool {
        self.read(self.ip) == 99
    }

    pub fn step(&mut self) {
        let ip = self.ip;
        let op1 = self.code[self.code[ip + 1] as usize];
        let op2 = self.code[self.code[ip + 2] as usize];
        let opcode = match self.code[ip] {
            1 => op1 + op2,
            2 => op1 * op2,
            _ => unreachable!(),
        };
        self.ip += 4;
        self.write(self.code[ip + 3] as usize, value);
    }

    pub fn execute(&mut self) {
        while !self.is_finished() {
            self.step()
        }
    }
}

#[aoc_generator(day2)]
fn get_input(data: &str) -> Process {
    Process {
        code: data.split(',').map(|n| n.parse::<u32>().unwrap()).collect(),
        ip: 0,
    }
}

#[aoc(day2, part1)]
fn part1(process: &Process) -> u32 {
    let mut process = process.folk();
    process.write(1, 12);
    process.write(2, 2);
    process.execute();
    process.code[0]
}

#[aoc(day2, part2)]
fn part2(process: &Process) -> u32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut p = process.folk();
            p.write(1, noun);
            p.write(2, verb);
            p.execute();
            if p.code[0] == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    unreachable!()
}
