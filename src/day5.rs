use std::collections::VecDeque;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<char> {
   
    input.chars().collect()
    
        
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[char]) -> i32 {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut pop: bool;
        for c in input {
            if let Some(o) = stack.front(){
                if c.eq_ignore_ascii_case(o) && c != o {
                    pop = true;
                } else {
                    pop = false;
                }
            } else {
                pop = false;
            }
            if pop {
                stack.pop_front();
            } else {
                stack.push_front(*c);
            }
        }
        stack.len() as i32
        
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[char]) -> i32 {
        let mut shortest = input.len();
        let alphabet = vec!('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut pop: bool;                           
        for c in alphabet {
            let mut chars: Vec<char> = input.to_vec();
            chars.retain(|r| {
                !c.eq_ignore_ascii_case(r)
            });
            
            for c in chars {
                if let Some(o) = stack.front(){
                    if c.eq_ignore_ascii_case(o) && c != *o {
                        pop = true;
                    } else {
                        pop = false;
                    }
                } else {
                    pop = false;
                }
                if pop {
                    stack.pop_front();
                } else {
                    stack.push_front(c);
                }
            }
            if stack.len() < shortest {
                shortest = stack.len();
            }
            stack.clear();
        }                           

       shortest as i32
}

