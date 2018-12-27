#[derive(Clone, Debug)]
pub enum FieldType { 
    Sand,
    Clay, 
    Water, 
    Dried 
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Vec<FieldType>> {
    let mut ranges: Vec<(usize, usize, usize, usize)> = input.lines().map(|l| {
        let parts: Vec<&str> = l.split(|c| c == '=' || c == ' ' || c == '.' || c == ',').collect();

        if parts[0] == "x" {
            (
                parts[1].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap(),
                parts[4].parse::<usize>().unwrap(), parts[6].parse::<usize>().unwrap()
            )
        } else {
            (
                parts[4].parse::<usize>().unwrap(), parts[6].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap()
            )
        }
    }).collect();
    ranges.sort();
    let sorted_ranges = ranges.clone();

    let (_, max_x, _, _) = sorted_ranges.last().unwrap();
    let max_y = sorted_ranges.iter().map(|(_, _, _, y)| y).max().unwrap();

    let mut board = vec![vec![FieldType::Sand; 1 + max_x]; 1 + max_y];

    for (x1, x2, y1, y2) in ranges {
        for x in x1..=x2 {
            for y in y1..=y2 {
                board[y][x] = FieldType::Clay;
            }
        }
    }

    board
}

fn flow(start_y: usize, x: usize, board: &mut Vec<Vec<FieldType>>) -> bool {
    for y in start_y..board.len() {
        if let FieldType::Sand = board[y][x] { board[y][x] = FieldType::Dried };

        if let FieldType::Dried = board[y][x] {
            board[y][x] = FieldType::Dried;
            continue;
        };

        let flow_at = y - 1;
        let mut filled = false;
        let mut overflows_left = true;
        let mut overflows_right = true;

        for steps in 1.. {
            let left = x - steps;

            if let FieldType::Clay = board[flow_at][left] {
                overflows_left = false;
                break;
            }

            board[flow_at][left] = FieldType::Dried;

            match board[y][left] {
                FieldType::Sand | FieldType::Dried => {
                    if flow(flow_at, left, board) { filled = true }
                    break;
                }
                _ => ()
            }
        }

        for steps in 0.. {
            let right = x + steps;

            if let FieldType::Clay = board[flow_at][right] {
                overflows_right = false;
                break;
            }

            board[flow_at][right] = FieldType::Dried;

            match board[y][right] {
                FieldType::Sand | FieldType::Dried => {
                    if flow(flow_at, right, board) { filled = true }
                    break;
                }
                _ => ()
            }
        }

        if !overflows_left && !overflows_right {
            filled = true;

            for steps in 1.. {
                let left = x - steps;

                if let FieldType::Dried = board[flow_at][left] {
                    board[flow_at][left] = FieldType::Water;
                } else { break; }
            }

            for steps in 0.. {
                let right = x + steps;

                if let FieldType::Dried = board[flow_at][right] {
                    board[flow_at][right] = FieldType::Water;
                } else { break; }
            }
        }

        return filled;
    }

    false
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Vec<Vec<FieldType>>) -> isize {
    let mut board = input.clone();


    loop {
        if !flow(0, 500, &mut board) {
            break;
        }
    }

    let mut sum = 0;
    board[0][500] = FieldType::Sand;
    let mut is_clay = false;

    for i in 0..(board.len()) {
        let row = &board[i];


        for field in row {
            match field {
                FieldType::Clay => is_clay = true,
                _ => ()
            }
        }

        if !is_clay { continue }

        for field in row {
            match field {
                FieldType::Water | FieldType::Dried => sum += 1,
                _other => ()
            }
        }
    }

    sum
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Vec<Vec<FieldType>>) -> isize {
    let mut board = input.clone();


    loop {
        if !flow(0, 500, &mut board) {
            break;
        }
    }

    let mut sum = 0;
    board[0][500] = FieldType::Sand;
    let mut is_clay = false;

    for i in 0..board.len() {
        let row = &board[i];


        for field in row {
            match field {
                FieldType::Clay => is_clay = true,
                _ => ()
            }
        }

        if !is_clay { continue }

        for field in row {
            match field {
                FieldType::Water => sum += 1,
                _other => ()
            }
        }
    }

    sum
}