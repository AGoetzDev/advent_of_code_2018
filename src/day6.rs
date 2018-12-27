
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Point {
    id: i32,
    x: i32,
    y: i32,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for (i, l) in input.lines().enumerate() {
        let split = l.split(", ").collect::<Vec<&str>>();
        result.push(Point {
            id: i as i32,
            x: split[0].parse::<i32>().unwrap(),
            y: split[1].parse::<i32>().unwrap(),
        });
    }
    result
        
}



#[aoc(day6, part1)]
pub fn solve_part1(input: &[Point]) -> i32 {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX; 
    let mut max_y = std::i32::MIN;
    for p in input {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.x < min_x {
            min_x = p.x
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }
    //calculate areas
    let mut dict_1 = calculate_areas(min_x, max_x, min_y, max_y, input);
    //calculate points that have infinite fields
    let infinite_points = calculate_infinite_points(min_x-1, max_x+1, min_y-1, max_y+1, input);
    //compare areas, find the maximum of points that aren't infinite
    let mut max_finite_area = std::i32::MIN;
    for p in input {
        let area_1 = dict_1.entry(p.id).or_insert(1);
        
        if !infinite_points.contains(&p.id) && *area_1 >  max_finite_area {
            max_finite_area = *area_1;
        }
    }
    max_finite_area
        
}

//iterate over all points on board and note down max area for each captured point
fn calculate_areas(min_x: i32, max_x:i32, min_y: i32, max_y: i32, points: &[Point]) -> HashMap<i32, i32> {
    let mut dict: HashMap<i32, i32> = HashMap::new();
    
    for x in min_x..max_x+1{
        for y in min_y..max_y+1 {
            if let Some(i) = get_closest_point(points, x, y) {
               let count = dict.entry(i).or_insert(0);
                *count+=1;
            }
        }
    }
    dict
}

//calculate which points expanded their area => infinite
fn calculate_infinite_points(min_x: i32, max_x:i32, min_y: i32, max_y: i32, points: &[Point]) -> HashSet<i32> {
    let mut dict: HashSet<i32> = HashSet::new();
    
    for x in min_x..max_x+1{
        //first row
        if let Some(i) = get_closest_point(points, x, min_y) {
            dict.insert(i);
        }
        //last row
        if let Some(i) = get_closest_point(points, x, max_y) {
            dict.insert(i);
        }
    }
    for y in min_y+1..max_y{
        //first column
        if let Some(i) = get_closest_point(points, min_x, y) {
            dict.insert(i);
        }
        //last colunm
        if let Some(i) = get_closest_point(points, max_x, y) {
            dict.insert(i);
        }
    }
    dict
}

//get the closest point for a coordinate
fn get_closest_point(points: &[Point], fixed_x: i32, fixed_y: i32) -> Option<i32> {
    let mut closest_id = 0;
    let mut multiple: bool = false;
    let mut closest_distance = std::i32::MAX;
    for p in points {
        let distance = (fixed_x-p.x).abs() + (fixed_y-p.y).abs();
        if distance == closest_distance {
            multiple = true;
        } else if distance < closest_distance {
            multiple = false;
            closest_id = p.id;
            closest_distance = distance;
         }
    }
    if !multiple {
            return Some(closest_id);
    }
    None
}




#[aoc(day6, part2)]
pub fn solve_part2(input: &[Point]) -> i32 {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX; 
    let mut max_y = std::i32::MIN;
    for p in input {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.x < min_x {
            min_x = p.x
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }
    //calculate areas
    let mut area = calculate_sum_distance_area(min_x, max_x, min_y, max_y, input, 10000);
    loop {
        //expand search area and calculate delta, stop if delta is 0
        min_x-=1;
        max_x+=1;
        min_y-=1;
        max_y+=1;
        let delta = calculate_sum_distance_area_delta(min_x, max_x, min_y, max_y, input, 10000);
        if delta == 0 {
            break;
        } else {
            area+=delta;
        }
    }
   area
        
}

fn calculate_sum_distance_area(min_x: i32, max_x:i32, min_y: i32, max_y: i32, points: &[Point], max_sum: i32) -> i32 {
    let mut count = 0;
    for x in min_x..max_x+1{
        for y in min_y..max_y+1 {
            let mut sum = 0;
            for p in points {
                 sum+= (x-p.x).abs() + (y-p.y).abs();
                
            }

            if sum < max_sum {
                count+=1;
            }
        }
    }
    count
}

fn calculate_sum_distance_area_delta(min_x: i32, max_x:i32, min_y: i32, max_y: i32, points: &[Point], max_sum: i32) -> i32 {
    let mut count = 0;
    for x in min_x..max_x+1{
        //first row
        let mut sum = 0;
        for p in points {
                sum+= (x-p.x).abs() + (min_y-p.y).abs();
                
        }
        if sum < max_sum {
                count+=1;
        }
        //last row
        sum = 0;
        for p in points {
                sum+= (x-p.x).abs() + (max_y-p.y).abs();
                
        }
        if sum < max_sum {
                count+=1;
        }
    }
    for y in min_y+1..max_y{
        //first column
        let mut sum = 0;
        for p in points {
                sum+= (min_x-p.x).abs() + (y-p.y).abs();
                
        }
        if sum < max_sum {
                count+=1;
        }
        //last colunm
        sum = 0;
        for p in points {
                sum+= (max_x-p.x).abs() + (y-p.y).abs();
                
        }
        if sum < max_sum {
                count+=1;
        }
    }
    count
}
