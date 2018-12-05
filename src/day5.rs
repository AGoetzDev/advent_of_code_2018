
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<char> {
   
    input.chars().collect()
    
        
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[char]) -> i32 {
        let mut chars: Vec<char> = input.to_vec();
        let mut index = 0;
 
        while index != chars.len(){
            if index != 0{
                if chars[index].eq_ignore_ascii_case(&chars[index-1]) && chars[index] != chars[index-1] {
                    chars.drain(index-1..index+1);
                    index-=2;
                }
            }
            index+=1;
        }
            
       chars.len() as i32
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[char]) -> i32 {
        let mut shortest = input.len();
        let alphabet = vec!('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
        for c in alphabet {
            let mut chars: Vec<char> = input.to_vec();
            chars.retain(|r| {
                !c.eq_ignore_ascii_case(r)
            });
            let mut index = 0;
            while index != chars.len(){
                if index != 0{
                    if chars[index].eq_ignore_ascii_case(&chars[index-1]) && chars[index] != chars[index-1] {
                        chars.drain(index-1..index+1);
                        index-=2;
                    }
                }
                index+=1;
            }
            if chars.len() < shortest {
                shortest = chars.len();
            }
        }                           

       shortest as i32
}

