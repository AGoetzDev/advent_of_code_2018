

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines().map(|s| String::from(s).chars().collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> i32 {
        let mut count_2 = 0;
        let mut count_3 = 0;
        let mut memory:[u32; 26] = [0; 26];
        let mut is_2 = false;
        let mut is_3 = false;
        for val in input.iter() {
                
                for c in val.iter(){
                        let count = memory[(*c as u32 -97) as usize]+1;
                        memory[(*c as u32 -97) as usize] = count;
                }
                for (i, value) in memory.iter_mut().enumerate() {
                      if *value == 2 {
                              is_2=true;
                      }  
                      if *value == 3 {
                              is_3=true;
                      }
                      *value = 0;  
                }
                if is_2 {
                        count_2 +=1;
                }
                if is_3 {
                        count_3 +=1;
                }
                is_2 = false;
                is_3 = false;
        }
        count_2*count_3
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> String {
        for val in input.iter() {
                for val2 in input.iter() {
                        match calculate_hamming(val, val2) {
                                Ok((distance, common)) => {
                                        if distance == 1{
                                                return common.iter().collect();
                                        }
                                },
                                Err(why) => panic!("{:?}", why)
                        }
                }
        }
        String::from("No match found")
}

fn calculate_hamming(input1: &Vec<char>, input2: &Vec<char>) -> Result<(i32, Vec<char>), &'static str>{
        if input1.len() != input2.len(){
                return Err("Strings must match in length");
        }
        let mut hamming = 0;
        let mut common_chars: Vec<char> = Vec::new();
        for (i, c) in input1.iter().enumerate() {
                if *c == input2[i] {
                        common_chars.push(*c);
                } else {
                        hamming+=1;
                }
        }
        Ok((hamming, common_chars))
}
