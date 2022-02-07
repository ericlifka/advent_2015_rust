use crate::intcode::IntcodeComputer;


pub fn run() {
    println!("2019:2");

    let mut computer = IntcodeComputer::new();

    computer.load_program("2019_02");
    computer.capture_state();

    computer.set(1, 12);
    computer.set(2, 2);
    let result = computer.run(0);
    println!("  part1: {}", result);

    for noun in 0..100 {
        for verb in 0..100 {
            computer.reset();
            computer.set(1, noun);
            computer.set(2, verb);
            let result = computer.run(0);
            if result == 19690720 {
                println!("  part2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}

