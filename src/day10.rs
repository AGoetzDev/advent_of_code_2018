use lazy_static::lazy_static;
use std::collections::HashSet;
use regex::Regex;

pub struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
impl Point {
    fn advance(&mut self){
        self.x += self.vx;
        self.y += self.vy;
    }
}

fn parse_point(input: &str) -> Point {
    lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                position=<\s*(?P<x>[-0-9]+),\s*(?P<y>[-0-9]+)>
                \s+
                velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>
            ").unwrap();
    }

        let caps = match RE.captures(input) {
            None => return Point{x: 0, y:0, vx: 0, vy:0},
            Some(caps) => caps,
        };
    Point {
            x: caps["x"].parse().unwrap_or(0),
            y: caps["y"].parse().unwrap_or(0),
            vx: caps["vx"].parse().unwrap_or(0),
            vy: caps["vy"].parse().unwrap_or(0),
    }
}

fn print(points: &[Point]) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();
    for p in points {
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
    let points_set: HashSet<_> = points.iter().map(|p| (p.x, p.y)).collect();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points_set.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn calculate_width(points: &[Point]) -> i32 {
  
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    for p in points {
        if p.x +p.vx > max_x {
            max_x = p.x +p.vx;
        }
        if p.x +p.vx < min_x {
            min_x = p.x +p.vx;
        }
    }

    max_x - min_x
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input.lines().map(|l| parse_point(l)).collect()
  
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Point]) -> u32 {
    let mut points_mut:Vec<Point> = Vec::new();
    for p in input {
        points_mut.push( Point{
            x: p.x,
            y:p.y,
            vx: p.vx,
            vy: p.vy,
        })
    }
    let mut width = i32::max_value();
    let mut seconds = 0;
    for i in 0.. {
        seconds = i;
        let n_width = calculate_width(&points_mut);
        if n_width > width {
            print(&points_mut);
            break;
        }
        width = n_width;
        for point in points_mut.iter_mut() {
           point.advance(); 
        }
    }
    
    seconds
        
}