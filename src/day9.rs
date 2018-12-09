use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let split: Vec<u32> = input.split_whitespace().map(|c| c.parse::<u32>().unwrap_or(0)).filter(|c| *c != 0).collect();
    split
        
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut players = vec![0; input[0] as usize];
    let max_marble = input[1];
    let mut current_player = 0;
    let mut ring = VecDeque::new();
    ring.push_back(0);

    for m in 1..max_marble+1 {
        if m % 23 == 0 {
            players[current_player] += m;
            for _i in 0..7 {
                let l = ring.pop_back().unwrap_or(0);
                ring.push_front(l);
            }
            players[current_player] += ring.pop_front().unwrap_or(0);

        } else {
            for _i in 0..2 {
                let f = ring.pop_front().unwrap_or(0);
                ring.push_back(f);
            }
            ring.push_front(m);
        }

        current_player = (current_player + 1) % players.len();
    }
    
    *players.iter().max().unwrap_or(&0)
        
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut players = vec![0; input[0] as usize];
    let max_marble = input[1]*100;
    let mut current_player = 0;
    let mut ring = VecDeque::new();
    ring.push_back(0);

    for m in 1..max_marble+1 {
        if m % 23 == 0 {
            players[current_player] += m;
            for _i in 0..7 {
                let l = ring.pop_back().unwrap_or(0);
                ring.push_front(l);
            }
            players[current_player] += ring.pop_front().unwrap_or(0);
        } else {
            for _i in 0..2 {
                let f = ring.pop_front().unwrap_or(0);
                ring.push_back(f);
            }
            ring.push_front(m);
        }
        current_player = (current_player + 1) % players.len();
    }

    *players.iter().max().unwrap_or(&0)
        
}