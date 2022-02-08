use crate::intcode::IntcodeComputer;

pub fn run() {
    let mut computer = IntcodeComputer::new(false);

    computer.load_program("2019_05");
    computer.add_to_input_buffer(1);
    computer.run(0);

    println!("  part1: {:?}", computer.empty_output_buffer());
}