use regex::Regex;

#[derive(Debug, Clone)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug, Clone)]
struct Program {
    registers: [usize; 6],
    instruction_pointer: usize,
    instructions: Vec<(Op, [usize; 3])>,
}

impl Program {
    fn do_op(&self, opcode: &Op, a: usize, b: usize) -> usize {
        match opcode {
            Op::Addr => self.registers[a] + self.registers[b],
            Op::Addi => self.registers[a] + b,
            Op::Mulr => self.registers[a] * self.registers[b],
            Op::Muli => self.registers[a] * b,
            Op::Banr => self.registers[a] & self.registers[b],
            Op::Bani => self.registers[a] & b,
            Op::Borr => self.registers[a] | self.registers[b],
            Op::Bori => self.registers[a] | b,
            Op::Setr => self.registers[a],
            Op::Seti => a,
            Op::Gtir => {
                if a > self.registers[b] { 1 } else { 0 }
            }
            Op::Gtri => {
                if self.registers[a] > b { 1 } else { 0 }
            }
            Op::Gtrr => {
                if self.registers[a] > self.registers[b] { 1 } else { 0 }
            }
            Op::Eqir => {
                if a == self.registers[b] { 1 } else { 0 }
            }
            Op::Eqri => {
                if self.registers[a] == b { 1 } else { 0 }
            }
            Op::Eqrr => {
                if self.registers[a] == self.registers[b] { 1 } else { 0 }
            }
        }
    }
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Program {
    let mut program = Program {
        registers: [0, 0, 0, 0, 0, 0],
        instruction_pointer: 0,
        instructions: Vec::new(),
    };
    let re = Regex::new(r"(\w{4})\s(\d+)\s(\d+)\s(\d+)").unwrap();
    for line in input.lines() {
        let bytes = line.as_bytes();
        if bytes[0] == b'#' {
            program.instruction_pointer = (bytes[4] as char).to_digit(10).unwrap() as usize;
            continue;
        }
        let caps = re.captures(line).unwrap();
        let instructions: [usize; 3] = [
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];
        let instruction = match &caps[1] {
            "addr" => (Op::Addr, instructions),
            "addi" => (Op::Addi, instructions),
            "mulr" => (Op::Mulr, instructions),
            "muli" => (Op::Muli, instructions),
            "banr" => (Op::Banr, instructions),
            "bani" => (Op::Bani, instructions),
            "borr" => (Op::Borr, instructions),
            "bori" => (Op::Bori, instructions),
            "setr" => (Op::Setr, instructions),
            "seti" => (Op::Seti, instructions),
            "gtir" => (Op::Gtir, instructions),
            "gtri" => (Op::Gtri, instructions),
            "gtrr" => (Op::Gtrr, instructions),
            "eqir" => (Op::Eqir, instructions),
            "eqri" => (Op::Eqri, instructions),
            "eqrr" => (Op::Eqrr, instructions),
            _ => unreachable!(),
        };
        program.instructions.push(instruction);
    }
    program
}


#[aoc(day19, part1)]
fn solve_part1(input_program: &Program) -> usize {
    let mut program = input_program.to_owned();
    let bound = program.instruction_pointer;
    program.instruction_pointer = program.registers[bound];
    while program.instruction_pointer < program.instructions.len() {
        program.registers[bound] = program.instruction_pointer;
        let instruction = &program.instructions[program.instruction_pointer];
        let [a, b, c] = instruction.1;
        let result = program.do_op(&instruction.0, a, b);
        program.registers[c] = result;
        program.instruction_pointer = program.registers[bound] + 1;
    }
    program.registers[0]
}


#[aoc(day19, part2)]
fn solve_part2(input_program: &Program) -> usize {
    let mut program = input_program.to_owned();
    let bound = program.instruction_pointer;
    program.registers[0] = 1;
    program.instruction_pointer = program.registers[bound];
    while program.registers[bound] != 1 {
        program.registers[bound] = program.instruction_pointer;
        let instruction = &program.instructions[program.instruction_pointer];
        let [a, b, c] = instruction.1;
        let result = program.do_op(&instruction.0, a, b);
        program.registers[c] = result;
        program.instruction_pointer = program.registers[bound] + 1;
    }

    let target = *program.registers.iter().max().unwrap();
    let mut total = 0;
    for i in 1..=target {
        if target % i == 0 {
            total += i;
        }
    }
    total
}