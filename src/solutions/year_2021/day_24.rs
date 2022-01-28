use crate::input;
use std::fmt::{Display,Formatter,Result};

#[derive(Debug, Copy, Clone)]
enum Symbol {
    W, X, Y, Z,
    Scaler(i64),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Inp(Symbol),
    Add(Symbol, Symbol),
    Mul(Symbol, Symbol),
    Div(Symbol, Symbol),
    Mod(Symbol, Symbol),
    Eql(Symbol, Symbol),
}

#[derive(Debug)]
struct ArithmeticLogicUnit<'a> {
    registers: [i64; 4],
    program: &'a Vec<Instruction>,
}

use Symbol::*;
use Instruction::*;

impl Symbol {
    fn new(declaration: &str) -> Symbol {
        match declaration {
            "w" => W, "x" => X, "y" => Y, "z" => Z,
             _  => Scaler(declaration.parse::<i64>().expect("couldn't parse number"))
        }
    }

    fn ordinal(&self) -> usize {
        match self {
            W => 0, X => 1, Y => 2, Z => 3,
            Scaler(_) => panic!("attempt to use scaler as ordinal"),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            W => write!(f, "w"),
            X => write!(f, "x"),
            Y => write!(f, "y"),
            Z => write!(f, "z"),
            Scaler(val) => write!(f, "{}", val),
        }
    }
}

impl Instruction {
    fn new(parts: Vec<&str>) -> Instruction {
        match parts[0] {
            "inp" => Inp(Symbol::new(parts[1])),
            "add" => Add(Symbol::new(parts[1]), Symbol::new(parts[2])),
            "mul" => Mul(Symbol::new(parts[1]), Symbol::new(parts[2])),
            "div" => Div(Symbol::new(parts[1]), Symbol::new(parts[2])),
            "mod" => Mod(Symbol::new(parts[1]), Symbol::new(parts[2])),
            "eql" => Eql(Symbol::new(parts[1]), Symbol::new(parts[2])),
            _ => { panic!("unexpected command"); },
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Inp(s) => write!(f, "inp {}", s),
            Add(s1, s2) => write!(f, "add {} {}", s1, s2),
            Mul(s1, s2) => write!(f, "mul {} {}", s1, s2),
            Div(s1, s2) => write!(f, "div {} {}", s1, s2),
            Mod(s1, s2) => write!(f, "mod {} {}", s1, s2),
            Eql(s1, s2) => write!(f, "eql {} {}", s1, s2),
        }
    }
}

impl ArithmeticLogicUnit<'_> {
    fn new<'a>(program: &'a Vec<Instruction>) -> ArithmeticLogicUnit<'a> {
        ArithmeticLogicUnit {
            registers: [0; 4],
            program,
        }
    }

    fn reset(&mut self) {
        self.registers = [0; 4];
    }

    fn run_program(&mut self, input_stream: [i64; 14], debug: bool) -> i64 {
        self.reset();

        if debug {
            println!("Start       {:3}|w {:12}|x {:4}|y {:12}|z",
                self.registers[0], self.registers[1], self.registers[2], self.registers[3]);
        }

        let program = self.program.iter();
        let mut input = input_stream.iter();
        let mut i = 1;

        for &instruction in program {
            match instruction {
                Inp(sym) => self.set_value(sym, *input.next().unwrap()),
                Add(sym1, sym2) => self.set_value(sym1, self.lookup_value(sym1) + self.lookup_value(sym2)),
                Mul(sym1, sym2) => self.set_value(sym1, self.lookup_value(sym1) * self.lookup_value(sym2)),
                Div(sym1, sym2) => self.set_value(sym1, self.lookup_value(sym1) / self.lookup_value(sym2)),
                Mod(sym1, sym2) => self.set_value(sym1, self.lookup_value(sym1) % self.lookup_value(sym2)),
                Eql(sym1, sym2) => self.set_value(sym1, 
                    if self.lookup_value(sym1) == self.lookup_value(sym2) { 1 } else { 0 }
                ),
            }

            if debug {
                if let Inp(_) = instruction {
                    println!("\n - {} -", i);
                    i += 1;
                }
                println!("{:10}->{:3}|w {:12}|x {:4}|y {:12}|z", instruction.to_string(),
                    self.registers[0], self.registers[1], self.registers[2], self.registers[3]);
            }
        }

        self.lookup_value(Z)
    }

    fn lookup_value(&self, sym: Symbol) -> i64 {
        match sym {
            W | X | Y | Z => self.registers[sym.ordinal()],
            Scaler(val) => val,
        }
    }

    fn set_value(&mut self, sym: Symbol, val: i64) {
        self.registers[sym.ordinal()] = val;
    }
}

pub fn run() {
    println!("2021:24");

    let program = read_program();
    let mut alu = ArithmeticLogicUnit::new(&program);

    let min_model_number = 11912814611156i64;
    let max_model_number = 45989929946199i64;

    println!("min: {} validation -> {}",
        min_model_number,
        alu.run_program(create_input_stream(min_model_number), false));

    println!("max: {} validation -> {}",
        max_model_number,
        alu.run_program(create_input_stream(max_model_number), false));
}

fn create_input_stream(mut input_num: i64) -> [i64; 14] {
    let mut input_stream = [0i64; 14];

    for i in (0..14).rev() {
        input_stream[i] = input_num % 10;
        input_num = input_num / 10;
    }

    input_stream
}

fn read_program() -> Vec<Instruction> {
    input::read_lines("2021_24_14")
        .expect("couldn't load program file")
        .iter()
        .map(|line| Instruction::new(line.split(" ").collect::<Vec<&str>>()))
        .collect::<Vec<Instruction>>()
}
/* **NOTES AREA**
# ruminations on base 26 trying to work out what the validation program is doing #

zzz -> dec
26**0 26**1 26**2
25*1 + 25*26 + 25*676
25 + 650 + 16900 = 17575

abcdefghij klmnopqrst uvwxyz
0123456789 0123456789 012345

      111 111
25**  543 210 987 654 321
0  ->                   1 ->                 21 -> v
1  ->                  26 ->                546 -> v
2  ->                 676 ->              8,112 -> m
3  ->              17,576 ->            404,248 -> x
4  ->             456,976 ->          6,397,664 -> o
5  ->          11,881,376 ->        249,508,896 -> v
6  ->         308,915,776 ->      3,706,989,312 -> m
7  ->       8,031,810,176 ->    176,699,823,872 -> w
8  ->     208,827,064,576 ->  2,088,270,645,760 -> k
9  ->   5,429,503,678,976 -> 97,731,066,221,568 -> s
-------------------------------------------------------
      100,000,000,000,000 |  99,999,999,999,999 = skwmvoxmvv

Tried to find biggest number I could fit in 14 digits, only to discover that would just be all 9s in decimal
and that 'skwmvoxmvv' has no meaning for the problem



# trying some rand words incase it's a code validator #

christmas (9)
479,231,987,226

santaclaus
97841350261426



# deriving digits from observed program execution #

From observation of different inputs I conclude that the program is the same fundamental block,
repeating 14 times, one for each digit in the input. There are two variations of the block.

The first, which I've dubbed the encoder block, the running total is shifted up one digit
in base 26 (multiplied by 26 in the decimal program values), then the input digit is read,
increased by an arbitrary constant, and added to the running total as the new lowest base 26 digit.

The second variation, dubbed the eliminator block, separates the lowest base 26 digit from the
running total while shifting the running total down a bit (essentially splitting the number).
The lowest digit is modified by an arbitrary constant and then compared to the input digit for the block.
If the digits match then no further action is taken, otherwise the running total is shifted back up a digit
and the input digit is added as the new loest digit. In effect the running total is either decreased by
one order of magnitude, or it's lowest digit is swapped for the newest input digit.

If the input digits are lined up with these two block types the running total can be controlled so
that it scales up through 5 base 26 orders of magnitude and then scales back down, dropping digits until
the final result is zero. If the numbers aren't aligned then the total scales forever and outputs
a huge base 26 number representing a modified version of the input number.

Working through the program, running subsequently larger numbers of blocks, this appears to be the
relationship created by the constants which when followed creates a valid model number by resulting
in zero in the running total (ranges are listed in base 26 first because as I was working through
this chart I still believed the base 26 representation of the numbers would be important somehow):

1  encodes range p-x (1-4) max 4 from 14                        +26**0
2  encodes range h-p (1-5) max 5 from 13                          +26**1
3  encodes range h-p (9-9) min 9 from 12                            +26**2
4  encodes range o-w (1-8) max 8 from 5                               +26**3
5  eliminates 4 if it's 1 more than 4, range  2 - 10 (2-9)            -26**3
6  encodes range j-r (8-9) min 8 from 7                               +26**3
7  eliminates 6 if it's 7 less than 6, range -6 -  2 (1-2)            -26**3
8  encodes range l-t (4-9) min from 11                                +26**3
9  encodes range j-r (6-9) min from 10                                  +26**4
10 eliminates 9 if it's 5 less than 9, range -4 -  4 (1-4)              -26**4
11 eliminates 8 if it's 3 less than 8, range -2 -  6 (1-6)            -26**3
12 eliminates 3 if it's 8 less than 3, range -7 -  1 (1)            -26**2
13 eliminates 2 if it's 4 more than 2, range  5 - 13 (5-9)        -26**1
14 eliminates 1 if it's 5 more than 1, range  6 - 14 (6-9)      -26**0

Distilled to a simplified formula and used to easily
pick the max and min input values that validate:
Digit   Range          Min     Max
  1     1-4             1       4
  2     1-5             1       5
  3      9              9       9
  4     1-8             1       8
  5    (4) + 1          2       9
  6     8-9             8       9
  7    (6) - 7          1       2
  8     4-9             4       9
  9     6-9             6       9
 10    (9) - 5          1       4
 11    (8) - 3          1       6
 12    (3) - 8          1       1
 13    (2) + 4          5       9
 14    (1) + 5          6       9
*/