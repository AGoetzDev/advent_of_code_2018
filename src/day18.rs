use std::collections::HashSet;
use std::collections::HashMap;

pub struct Board {
    fields: Vec<Vec<AcreContent>>,
}

impl AsRef<Board> for Board {
    fn as_ref(&self) -> &Board {
        self
    }
}

#[derive(Debug, Eq, Clone, Copy, PartialEq, Hash)]
pub enum AcreContent {
    Lumberyard,
    Ground,
    Tree,
}

impl AcreContent {
    fn fmt(&self) -> char {
        match self {
            AcreContent::Ground => '.',
            AcreContent::Lumberyard => '#',
            AcreContent::Tree => '|',
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Board {
    let mut fields: Vec<Vec<AcreContent>> = Vec::new();

    for (_y, l) in input.lines().enumerate() {
        let mut fields_row: Vec<AcreContent> = Vec::new();
        for (_x, c) in l.chars().enumerate() {
            match c {
                '#' => fields_row.push(AcreContent::Lumberyard),
                '.' => fields_row.push(AcreContent::Ground),
                '|' => fields_row.push(AcreContent::Tree),
                _ => fields_row.push(AcreContent::Ground),
            }
        }
        fields.push(fields_row);
    }
    Board { fields: fields }
}


fn find_changeset(
    current_board: &mut Vec<Vec<AcreContent>>,
    changes: &mut Vec<((usize, usize), AcreContent)>,
    max: (usize, usize)
) -> HashSet<(usize, usize)>{
    let mut area_count: HashMap<AcreContent, usize> = HashMap::new();
    let mut change_set: HashSet<(usize, usize)> = HashSet::new();
    for (y, _row) in current_board.iter().enumerate() {
        for (x, acre) in current_board[y].iter().enumerate() {
            area_count.clear();
            area_count.entry(AcreContent::Tree).or_insert(0);
            area_count.entry(AcreContent::Lumberyard).or_insert(0);
            area_count.entry(AcreContent::Ground).or_insert(0);
            for oy in -1..=1 {
                for ox in -1..=1 {
                    let (c_y, c_x) = (y as i32 - oy, x as i32 - ox);
                    if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x < max.1 as i32 {
                        *area_count.entry(current_board[c_y as usize][c_x as usize]).or_insert(0) += 1;
                    }
                    
                }
            }

            match acre {
                AcreContent::Ground => {
                    if area_count[&AcreContent::Tree] >= 3 {
                        changes.push(((y, x), AcreContent::Tree));
                        for t_y in -1..2 {
                            for t_x in -1..2 {
                                let c_y = y as i32 + t_y;
                                let c_x = x as i32 + t_x;

                                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x <max.1 as i32 {
                                    change_set.insert((c_y as usize, c_x as usize));
                                }
                            }
                        }
                    }
                }
                AcreContent::Tree => {
                    if area_count[&AcreContent::Lumberyard] >= 3 {
                        changes.push(((y, x), AcreContent::Lumberyard));
                        for t_y in -1..2 {
                            for t_x in -1..2 {
                                let c_y = y as i32 + t_y;
                                let c_x = x as i32 + t_x;

                                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x <max.1 as i32 {
                                    change_set.insert((c_y as usize, c_x as usize));
                                }
                            }
                        }
                    }
                }
                AcreContent::Lumberyard => {
                    if area_count[&AcreContent::Tree] < 1 || area_count[&AcreContent::Lumberyard] < 2{
                        changes.push(((y, x), AcreContent::Ground));
                        for t_y in -1..2 {
                            for t_x in -1..2 {
                                let c_y = y as i32 + t_y;
                                let c_x = x as i32 + t_x;

                                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x <max.1 as i32 {
                                    change_set.insert((c_y as usize, c_x as usize));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for ((y, x), content) in changes.iter(){
        
        current_board[*y][*x] = content.clone();
    }
    change_set
}

fn do_step(
    current_board: &mut Vec<Vec<AcreContent>>,
    changes: &mut Vec<((usize, usize), AcreContent)>,
    change_set: &mut HashSet<(usize, usize)>,
    max: (usize, usize),
) -> HashSet<(usize, usize)>{
    let mut area_count: HashMap<AcreContent, usize> = HashMap::new();
    let mut new_change_set: HashSet<(usize, usize)> = HashSet::new();
    for (y, x) in change_set.iter() {
        let acre = &current_board[*y][*x];
        
        area_count.clear();
        area_count.entry(AcreContent::Tree).or_insert(0);
        area_count.entry(AcreContent::Lumberyard).or_insert(0);
        area_count.entry(AcreContent::Ground).or_insert(0);
        for oy in -1..=1 {
             for ox in -1..=1 {
                let (c_y, c_x) = (*y as i32 - oy, *x as i32 - ox);
                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x < max.1 as i32 {
                    *area_count.entry(current_board[c_y as usize][c_x as usize]).or_insert(0) += 1;
                }
                    
            }
        }
            match acre {
                AcreContent::Ground => {
                    if area_count[&AcreContent::Tree] >= 3 {
                        changes.push(((*y, *x), AcreContent::Tree));
                        for t_y in -1..2 {
                            for t_x in -1..2 {
                                let c_y = *y as i32 + t_y;
                                let c_x = *x as i32 + t_x;

                                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x <max.1 as i32 {
                                    new_change_set.insert((c_y as usize, c_x as usize));
                                }
                            }
                        }
                    }
                }
                AcreContent::Tree => {
                    if area_count[&AcreContent::Lumberyard] >= 3 {
                        changes.push(((*y, *x), AcreContent::Lumberyard));
                        for t_y in -1..2 {
                            for t_x in -1..2 {
                                let c_y = *y as i32 + t_y;
                                let c_x = *x as i32 + t_x;

                                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x <max.1 as i32 {
                                    new_change_set.insert((c_y as usize, c_x as usize));
                                }
                            }
                        }
                    }
                }
                AcreContent::Lumberyard => {
                    if area_count[&AcreContent::Tree] < 1 || area_count[&AcreContent::Lumberyard] < 2{
                        changes.push(((*y, *x), AcreContent::Ground));
                        for t_y in -1..2 {
                            for t_x in -1..2 {
                                let c_y = *y as i32 + t_y;
                                let c_x = *x as i32 + t_x;

                                if c_y >= 0 && c_y < max.0 as i32 && c_x >= 0 && c_x <max.1 as i32 {
                                    new_change_set.insert((c_y as usize, c_x as usize));
                                }
                            }
                        }
                    }
            }
        }
    }
    for ((y, x), content) in changes.iter(){
        
        current_board[*y][*x] = content.clone();
    }
    changes.clear();
    new_change_set
}

#[aoc(day18, part1)]
pub fn solve_part1(board: &Board) -> i32 {
    let mut current_board = board.fields.clone();
    let mut changes: Vec<((usize, usize), AcreContent)> = Vec::new();
    
    let max_y = current_board.len();
    let max_x = current_board[0].len();
    let mut change_set: HashSet<(usize, usize)> = find_changeset(&mut current_board, &mut changes, (max_y, max_x));

   
    for _i in 1..10 {
       
        change_set = do_step(&mut current_board, &mut changes, &mut change_set,  (max_y, max_x));

    }

    let mut num_tree = 0;
    let mut num_ly = 0;
    for (y, _row) in current_board.iter().enumerate() {
        for (_x, acre) in current_board[y].iter().enumerate() {
            if *acre == AcreContent::Tree {
                num_tree += 1;
            }
            if *acre == AcreContent::Lumberyard {
                num_ly += 1;
            }
        }
    }
    num_tree * num_ly
}


#[aoc(day18, part2)]
pub fn solve_part2(board: &Board) -> i32 {
    let mut current_board = board.fields.clone();
    let mut changes: Vec<((usize, usize), AcreContent)> = Vec::new();
    
    let max_y = current_board.len();
    let max_x = current_board[0].len();
    let mut change_set: HashSet<(usize, usize)> = find_changeset(&mut current_board, &mut changes, (max_y, max_x));

    let mut num_tree = 0;
    let mut num_ly = 0;
    let mut seen = HashMap::new();
    for i in 1..1000000000 {

        if seen.contains_key(&current_board) {
            let rest = (1_000_000_000 - i) % (i - seen[&current_board]);
            for _ in 0..rest {
                change_set = do_step(&mut current_board, &mut changes, &mut change_set, (max_y, max_x));
            }
            break;
        }
        seen.insert(current_board.clone(), i);
        change_set = do_step(&mut current_board, &mut changes, &mut change_set, (max_y, max_x));
       
        
    }

    for (y, _row) in current_board.iter().enumerate() {
        for (_x, acre) in current_board[y].iter().enumerate() {
            if *acre == AcreContent::Tree {
                 num_tree += 1;
            }
            if *acre == AcreContent::Lumberyard {
                num_ly += 1;
            }
        }
    }
   
    num_tree * num_ly
}
