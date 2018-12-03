

pub struct  FabricClaim{
        id: i32,
        x: i32,
        y: i32,
        w: i32, 
        h: i32
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<FabricClaim> {
   
    input
        .lines().map(|s| {
             parse_claim(s)
        })
        .collect()
        
}

enum ParseState {
        X,
        Y,
        W,
        H,
        Id,
        None,
}

fn parse_claim(input: &str)  -> FabricClaim{
        let mut state: ParseState = ParseState::None;
        let mut string_x: String = String::new();
        let mut string_y: String = String::new();
        let mut string_w: String = String::new();
        let mut string_h: String = String::new();
        let mut string_id: String = String::new();
        for c in input.chars(){
              match c {
                        '@' => state = ParseState::X,
                        ',' => state = ParseState::Y,
                        ':' => state = ParseState::W,
                        'x' => state = ParseState::H,
                        '#' => state = ParseState::Id,
                        _ => {
                              match state {
                                    ParseState::X => {
                                            if c != ' ' {
                                                    string_x.push(c);
                                            }
                                    },
                                    ParseState::Y => {
                                            if c != ' ' {
                                                    string_y.push(c);
                                            }
                                    }
                                    ParseState::W => {
                                            if c != ' ' {
                                                    string_w.push(c);
                                            }
                                    },
                                    ParseState::H => {
                                            if c != ' ' {
                                                    string_h.push(c);
                                            }
                                    }, 
                                    ParseState::Id => {
                                            if c != ' ' {
                                                    string_id.push(c);
                                            }
                                    }, 
                                    ParseState::None => {

                                    }       
                              }
                      }
              } 
        }
        
        FabricClaim {
                id: string_id.parse::<i32>().unwrap(),
                x: string_x.parse::<i32>().unwrap(),
                y: string_y.parse::<i32>().unwrap(),
                w: string_w.parse::<i32>().unwrap(),
                h: string_h.parse::<i32>().unwrap(),
        }

}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[FabricClaim]) -> i32 {
        let mut count = 0;
        let mut max_h = 0;
        let mut max_w = 0;
        //calculate fabric dimensions
        for claim in input {
                if  (claim.x+claim.w) > max_w {
                     max_w = claim.x+claim.w;
                }
                if  (claim.y+claim.h) > max_h {
                     max_h = claim.y+claim.h;
                } 
        }
        max_h+=1;
        max_w+=1;

        let mut fabric: Vec<Vec<i32>> = (0..(max_w)).map(|_| vec!(0;(max_h) as usize)).collect();
        //mark fabric
        for claim in input {
                for x in claim.x..(claim.x+claim.w) {
                        for y in claim.y..(claim.y+claim.h) {
                                fabric[x as usize][y as usize]  +=1;
                        }
                } 
        }
        //count which squares are overlapping
        for x in 0..max_w {
                for y in 0..max_h {
                        if fabric[x as usize][y as usize] > 1 {
                                count+=1;
                        }
                }
        }
        
        count
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[FabricClaim]) -> i32 {
        let mut max_h = 0;
        let mut max_w = 0;
        //calculate fabric dimensions
        for claim in input {
                if  (claim.x+claim.w) > max_w {
                     max_w = claim.x+claim.w;
                }
                if  (claim.y+claim.h) > max_h {
                     max_h = claim.y+claim.h;
                } 
        }
        max_h+=1;
        max_w+=1;

        
        let mut fabric: Vec<Vec<i32>> = (0..(max_w)).map(|_| vec!(0;(max_h) as usize)).collect();
        //fill fabric
        for claim in input {
                for x in claim.x..(claim.x+claim.w) {
                        for y in claim.y..(claim.y+claim.h) {
                                fabric[x as usize][y as usize]  +=1;
                        }
                } 
        }
        //check which claim only has 1 => is not overlapping
        let mut untouched: bool;
        for claim in input {
                untouched = true;
                for x in claim.x..(claim.x+claim.w) {
                        for y in claim.y..(claim.y+claim.h) {
                                if fabric[x as usize][y as usize] > 1 {
                                        untouched = false;
                                }
                        }
                } 
                if untouched {
                        return claim.id;
                }
        }
        
        0
}
