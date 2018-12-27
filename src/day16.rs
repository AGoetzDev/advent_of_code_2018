use std::collections::HashMap;
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

impl Op {
    fn values() -> Vec<Op> {
        vec![
            Op::Addr,
            Op::Addi,
            Op::Mulr,
            Op::Muli,
            Op::Banr,
            Op::Bani,
            Op::Borr,
            Op::Bori,
            Op::Setr,
            Op::Seti,
            Op::Gtir,
            Op::Gtri,
            Op::Gtrr,
            Op::Eqir,
            Op::Eqri,
            Op::Eqrr,
        ]
    }
}

/// (before Before, instruction, before After)
type Sample = (Vec<usize>, Vec<usize>, Vec<usize>);
type Day16Input = (Vec<Sample>, Vec<Vec<usize>>);

fn do_op(opcode: Op, register: &[usize], instruction: &[usize]) -> usize {
    match opcode {
       Op::Addr => register[instruction[1]] + register[instruction[2]],
        Op::Addi => register[instruction[1]] + instruction[2],
        Op::Mulr => register[instruction[1]] * register[instruction[2]],
        Op::Muli => register[instruction[1]] * instruction[2],
        Op::Banr => register[instruction[1]] & register[instruction[2]],
        Op::Bani => register[instruction[1]] & instruction[2],
        Op::Borr => register[instruction[1]] | register[instruction[2]],
        Op::Bori => register[instruction[1]] | instruction[2],
        Op::Setr => register[instruction[1]],
        Op::Seti => instruction[1],
       Op::Gtir => {
            if instruction[1] > register[instruction[2]] { 1 } else { 0 }
        }
        Op::Gtri => {
            if register[instruction[1]] > instruction[2] { 1 } else { 0 }
        }
        Op::Gtrr => {
            if register[instruction[1]] > register[instruction[2]] { 1 } else { 0 }
        }
        Op::Eqir => {
            if instruction[1] == register[instruction[2]] { 1 } else { 0 }
        }
        Op::Eqri => {
            if register[instruction[1]] == instruction[2] { 1 } else { 0 }
        }
        Op::Eqrr => {
            if register[instruction[1]] == register[instruction[2]] { 1 } else { 0 }
        }
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Day16Input {
    let re: Regex = Regex::new(r".+:\s{1,2}\[(\d{1}), (\d{1}), (\d{1}), (\d{1})\]").unwrap();
    let mut samples: Vec<Sample> = Vec::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    // Get Samples
    for lines in lines.chunks(4) {
        if !lines[0].contains("Before:") {
            break;
        }

        let before_groups = re.captures(lines[0]).unwrap();
        let before: Vec<usize> = before_groups
            .iter()
            .skip(1)
            .take(4)
            .map(|v| v.unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        let instruction: Vec<usize> = lines[1]
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let after_groups = re.captures(lines[2]).unwrap();
        let after: Vec<usize> = after_groups
            .iter()
            .skip(1)
            .take(4)
            .map(|v| v.unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        samples.push((before, instruction, after));
    }

    // Get Test Code
    for lines in lines.iter().skip(samples.len() * 4) {
        if lines.is_empty() {
            continue;
        }

        let instruction: Vec<usize> = lines
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        instructions.push(instruction);
    }

    (samples, instructions)
}

#[aoc(day16, part1)]
fn solve_part1(input: &Day16Input) -> usize {
    let mut total_count = 0;
    for sample in &input.0 {
        let mut sample_count = 0;
        let (before, instruction, after) = sample;
        for opcode in 0..16 {
            let result = do_op(Op::values()[opcode].clone(), before, instruction);
            let mut new_after = before.to_owned();
            new_after[instruction[3]] = result;
            if new_after == *after {
                sample_count += 1;
            }
        }
        if sample_count >= 3 {
            total_count += 1;
        }
    }
    total_count
}

#[aoc(day16, part2)]
fn solve_part2(input: &Day16Input) -> usize {
    let mut candidates = Vec::new();
    for opcode in &Op::values() {
        let mut possibilities = Vec::new();
        for n in 0..16 {
            let missed = input.0.iter().filter(|s| s.1[0] == n).any(|s| {
                let result = do_op(opcode.clone(), &s.0, &s.1);
                let mut new_after = s.0.to_owned();
                new_after[s.1[3]] = result;
                new_after != s.2
            });
            if !missed {
                possibilities.push((n, opcode.clone()));
            }
        }
        candidates.push(possibilities);
    }
    let mut opcodes = HashMap::new();
    while opcodes.len() < 16 {
        for possibilities in &candidates {
            let new: Vec<(usize, Op)> = possibilities
                .iter()
                .cloned()
                .filter(|p| !opcodes.contains_key(&p.0))
                .collect();
            if new.len() == 1 {
                opcodes.insert(new[0].0, new[0].1.clone());
            }
        }
    }

    let mut registers = vec![0, 0, 0, 0];
    for instruction in &input.1 {
        if let Some(opcode) = opcodes.get(&instruction[0]) {
            let result = do_op(opcode.clone(), &registers, instruction);
            registers[instruction[3]] = result;
        }
    }
    registers[0]
}

