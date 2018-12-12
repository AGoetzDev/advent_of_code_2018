#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Box<i32> {
    Box::new(input.parse::<i32>().unwrap())
        
}

fn calculate_power(x: i32, y: i32, serial: i32) ->  i32{
    let rack_id = x+10;
    let mut p = rack_id * y + serial;
    p *=rack_id;
    if p > 100 {
         p = ((p % 1000 - p % 100) / 100).abs();
    } else {
        p = 0;
    }
   
    p -= 5;
    p
}


#[aoc(day11, part1)]
pub fn solve_part1(input: &i32) -> i32 {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_area = i32::min_value();
    let mut board = vec![vec![0i32; 300]; 300];
    let serial: i32 = *input;
     for x in 0..300 {
         for y in 0 ..300 {
             board[x][y] = calculate_power(x as i32 +1, y as i32 +1, serial);
         }
     }

    for x in 0 ..298{
        for y in 0..298 {
            let sum = board[x][y]+board[x+1][y]+board[x+2][y]
                     + board[x][y+1]+board[x+1][y+1]+board[x+2][y+1]
                     + board[x][y+2]+board[x+1][y+2]+board[x+2][y+2];
            if sum > max_area {
                max_area = sum;
                max_x = x;
                max_y = y;
            }
        }
    }
   
    println!("{}, {}", max_x+1, max_y+1);
    max_area
     
        
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &i32) -> i32 {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_area = i32::min_value();
    let mut board = vec![vec![0i32; 300]; 300];
    let serial: i32 = *input;
     for x in 0..300 {
         for y in 0 ..300 {
             board[x][y] = calculate_power(x as i32 +1, y as i32 +1, serial);
         }
     }
    //Ugly bruteforce will fix later
    let mut w_result = 0;
    for window_size in 0..299 {
        for x in 0 ..300-window_size{
            for y in 0..300-window_size {
                let mut sum = 0;
                for w_x in x..x+window_size+1{
                    for w_y in y..y+window_size+1{
                        sum+=board[w_x][w_y];
                    }
                }
                
                if sum > max_area {
                    max_area = sum;
                    max_x = x;
                    max_y = y;
                    w_result = window_size;
                }
            }
        }
    } 
    
    println!("{},{},{}", max_x+1, max_y+1, w_result+1);
    max_area
     
        
}