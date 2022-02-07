use crate::input;

#[derive(Debug)]
pub struct IntcodeComputer {
    memory: Vec<i64>,
    instruction_ptr: i64,
    save_state: Option<Box<IntcodeComputer>>,
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            memory: Vec::new(),
            instruction_ptr: 0,
            save_state: None,
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
            save_state: None,
        }));
    }

    pub fn reset(&mut self) {
        match &self.save_state {
            None => {
                self.memory = Vec::new();
                self.instruction_ptr = 0;
            },
            Some(saved) => {
                self.memory = saved.memory.to_vec();
                self.instruction_ptr = saved.instruction_ptr;
            },
        }

    }

    pub fn lookup(&self, index: i64) -> i64 {
        if index < 0 || index >= self.memory.len().try_into().unwrap() {
            panic!("Tried to access index out of memory range");
        }
        
        self.memory[index as usize]
    }

    pub fn lookup_3(&self, start: i64) -> (i64, i64, i64) {
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

        match self.lookup(ptr) {
            1 => {
                let (in1, in2, out) = self.lookup_3(ptr + 1);

                let lval = self.lookup(in1);
                let rval = self.lookup(in2);
                let result = lval + rval;

                self.set(out, result);
                self.instruction_ptr += 4;
            },

            2 => {
                let (in1, in2, out) = self.lookup_3(ptr + 1);

                let lval = self.lookup(in1);
                let rval = self.lookup(in2);
                let result = lval * rval;

                self.set(out, result);
                self.instruction_ptr += 4;
            },

            99 => return false,
            _  => panic!("Unexpected opcode"),
        }

        true
    }
}