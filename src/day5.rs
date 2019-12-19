use crate::intcode::Process;
#[aoc_generator(day5)]
fn get_input(data: &str) -> Process {
    Process::new(data)
}

#[aoc(day5, part1)]
fn part1(process: &Process) -> i32 {
    let mut process = process.folk();
    process.inputs.push(1);
    process.execute();
    *process.outputs.last().unwrap()
}

#[aoc(day5, part2)]
fn part2(process: &Process) -> i32 {
    let mut process = process.folk();
    process.inputs.push(5);
    process.execute();
    println!("{:?}", process.outputs);
    *process.outputs.last().unwrap()
}
