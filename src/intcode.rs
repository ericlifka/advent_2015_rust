use crate::input;

#[derive(Debug)]
pub struct IntcodeComputer {
    memory: Vec<i64>,
    instruction_ptr: i64,
    input_buffer: Vec<i64>,
    output_buffer: Vec<i64>,

    save_state: Option<Box<IntcodeComputer>>,
    debug: bool,
}

impl IntcodeComputer {
    pub fn new(debug: bool) -> IntcodeComputer {
        IntcodeComputer {
            memory: Vec::new(),
            instruction_ptr: 0,
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
            save_state: None,
            debug,
        }
    }

    pub fn load_program(&mut self, filename: &str) {
        let text = input::read_all(filename).expect("couldn't load program file");
        
        for num in text.split(",").map(|n| n.parse::<i64>().unwrap()) {
            self.memory.push(num);
        }
    }

    pub fn capture_state(&mut self) {
        self.save_state = Some(Box::new(IntcodeComputer {
            memory: self.memory.to_vec(),
            instruction_ptr: self.instruction_ptr,
            input_buffer: self.input_buffer.to_vec(),
            output_buffer: self.output_buffer.to_vec(),
            save_state: None,
            debug: false,
        }));
    }

    pub fn reset(&mut self) {
        match &self.save_state {
            None => {
                self.memory = Vec::new();
                self.instruction_ptr = 0;
                self.input_buffer = Vec::new();
                self.output_buffer = Vec::new();
            },
            Some(saved) => {
                self.memory = saved.memory.to_vec();
                self.instruction_ptr = saved.instruction_ptr;
                self.input_buffer = saved.input_buffer.to_vec();
                self.output_buffer = saved.output_buffer.to_vec();
            },
        }
    }

    pub fn lookup(&self, index: i64) -> i64 {
        if index < 0 || index >= self.memory.len().try_into().unwrap() {
            panic!("Tried to access index out of memory range");
        }
        
        self.memory[index as usize]
    }

    pub fn add_to_input_buffer(&mut self, input: i64) {
        self.input_buffer.push(input);
    }

    pub fn empty_output_buffer(&mut self) -> Vec<i64> {
        let output = self.output_buffer.to_vec();
        self.output_buffer = Vec::new();

        output
    }

    fn lookup_3(&self, start: i64) -> (i64, i64, i64) {
        let one = self.lookup(start);
        let two = self.lookup(start + 1);
        let thr = self.lookup(start + 2);

        (one, two, thr)
    }

    pub fn set(&mut self, index: i64, value: i64) {
        if index < 0 || index >= self.memory.len().try_into().unwrap() {
            panic!("Tried to access index out of memory range");
        }

        self.memory[index as usize] = value;
    }

    pub fn run(&mut self, result_at: i64) -> i64 {
        while self.run_instruction() {}

        self.lookup(result_at)
    }

    fn run_instruction(&mut self) -> bool {
        let ptr = self.instruction_ptr;
        let instruction = self.lookup(ptr);
        let opcode = instruction % 100;
        let modes = instruction / 100;

        if self.debug {
            println!("{}: {}", ptr, instruction);
        }

        match opcode {
            1 => {
                let (p1, p2, p3) = self.lookup_3(ptr + 1);
                let (m1, m2) = calc_two_modes(modes);

                let lval = if m1 == 1 { p1 } else { self.lookup(p1) };
                let rval = if m2 == 1 { p2 } else { self.lookup(p2) };
                let result = lval + rval;

                self.set(p3, result);
                self.instruction_ptr += 4;
            },

            2 => {
                let (p1, p2, p3) = self.lookup_3(ptr + 1);
                let (m1, m2) = calc_two_modes(modes);

                let lval = if m1 == 1 { p1 } else { self.lookup(p1) };
                let rval = if m2 == 1 { p2 } else { self.lookup(p2) };
                let result = lval * rval;

                self.set(p3, result);
                self.instruction_ptr += 4;
            },

            3 => {
                let input = self.input_buffer.remove(0);
                let p1 = self.lookup(ptr + 1);

                self.set(p1, input);
                self.instruction_ptr += 2;
            },

            4 => {
                let p1 = self.lookup(ptr + 1);
                let m1 = calc_one_mode(modes);

                let val = if m1 == 1 { p1 } else { self.lookup(p1) };

                self.output_buffer.push(val);
                self.instruction_ptr += 2;
            },

            99 => return false,
            _  => panic!("Unexpected opcode"),
        }

        true
    }
}

fn calc_one_mode(modes: i64) -> i64 {
    modes % 10
}

fn calc_two_modes(mut modes: i64) -> (i64, i64) {
    let m1 = modes % 10;
    modes = modes / 10;
    let m2 = modes % 10;

    (m1, m2)
}
