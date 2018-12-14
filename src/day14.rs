#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Box<u32> {
    Box::new(input.parse::<u32>().unwrap())
    
}

fn number_to_vec(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

#[aoc(day14, part1)]
pub fn  solve_part1(input: &u32) -> u64 {
    let mut recipes: Vec<u32> = vec!(3,7);
    let mut elf_1: usize = 0;
    let mut elf_2: usize = 1;
    let max_length = *input as usize +10;
    while recipes.len() != max_length {
        let sum = recipes[elf_1] + recipes[elf_2];
        for n in number_to_vec(sum) {
            recipes.push(n);
        }
        elf_1 = (elf_1+(1+recipes[elf_1] as usize)) % recipes.len() ;
        elf_2 = (elf_2+(1+recipes[elf_2]) as usize) % recipes.len() ;
    }

    recipes[*input as usize..recipes.len()]
        .iter()
        .fold(0, |digits, &score| digits * 10 + score as u64)
    
    
}

#[aoc(day14, part2)]
pub fn  solve_part2(input: &u32) -> u64 {
    let mut recipes: Vec<u32> = vec!(3,7);
    let mut elf_1: usize = 0;
    let mut elf_2: usize = 1;
    let goal = number_to_vec(*input);
    loop {
        if recipes.ends_with(&goal) {
            return (recipes.len() - goal.len()) as u64;
        } else if recipes[..recipes.len()-1].ends_with(&goal) {
            return (recipes.len() - goal.len() - 1) as u64;
        }

        let sum = recipes[elf_1] + recipes[elf_2];
        for n in number_to_vec(sum as u32) {
            recipes.push(n);
        }
        elf_1 = (elf_1+(1+recipes[elf_1] as usize)) % recipes.len() ;
        elf_2 = (elf_2+(1+recipes[elf_2]) as usize) % recipes.len() ;
    }
 
}

