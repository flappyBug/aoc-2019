use std::convert::TryInto;

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

    pub fn read<T: TryInto<usize>>(&self, addr: T) -> u32 {
        self.code[addr.try_into().ok().unwrap()]
    }

    pub fn indirect_read<T: TryInto<usize>>(&self, addr: T) -> u32 {
        self.read(self.read(addr))
    }

    pub fn write<T: TryInto<usize>, V: TryInto<u32>>(&mut self, addr: T, value: V) {
        self.code[addr.try_into().ok().unwrap()] = value.try_into().ok().unwrap()
    }

    pub fn is_finished(&self) -> bool {
        self.read(self.ip) == 99
    }

    pub fn step(&mut self) {
        let ip = self.ip;
        let op1 = self.indirect_read(ip + 1);
        let op2 = self.indirect_read(ip + 2);
        let value = match self.code[ip] {
            1 => op1 + op2,
            2 => op1 * op2,
            _ => unreachable!(),
        };
        self.ip += 4;
        self.write(self.code[ip + 3], value);
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
