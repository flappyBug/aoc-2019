use crate::intcode::Process;
use itertools::Itertools;

#[aoc_generator(day7)]
fn get_input(data: &str) -> Process {
    Process::new(data)
}

struct Amplifier {
    program: Process,
    phase: i32,
}

impl Amplifier {
    fn run(&mut self, input: i32) -> i32 {
        // let mut process = self.program.folk();
        self.program.inputs.push(input);
        self.program.inputs.push(self.phase);
        self.program.execute();
        assert_eq!(self.program.outputs.len(), 1);
        self.program.outputs[0]
    }
}

struct AmpChain {
    amps: Vec<Amplifier>,
}

impl AmpChain {
    fn new(program: &Process, phases: Vec<i32>) -> Self {
        let amps = phases
            .into_iter()
            .map(|phase| Amplifier {
                program: program.folk(),
                phase,
            })
            .collect();
        Self { amps }
    }

    fn output(&mut self) -> i32 {
        self.amps.iter_mut().fold(0, |input, amp| amp.run(input))
    }
}

#[aoc(day7, part1)]
fn part1(program: &Process) -> i32 {
    (0..=4)
        .permutations(5)
        .map(|phases| AmpChain::new(program, phases).output())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_given_phase() {
        let data = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let program = get_input(data);
        let mut amp_chain = AmpChain::new(&program, vec![1, 0, 4, 3, 2]);
        assert_eq!(amp_chain.output(), 65210);
    }

    #[test]
    fn test_find_max_output_phase() {
        let data = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let program = get_input(data);
        assert_eq!(part1(&program), 65210);
    }
}
