use crate::intcode::Process;

#[aoc_generator(day2)]
fn get_input(data: &str) -> Process {
    Process::new(data)
}

#[aoc(day2, part1)]
fn part1(process: &Process) -> i32 {
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
