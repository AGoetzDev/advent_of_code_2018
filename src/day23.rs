use std::collections::HashMap;
use std::collections::BTreeMap;
use regex::Regex;


#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Coord(isize, isize, isize);

/// pos=<75543860,72403174,16415803>, r=92408569
#[aoc_generator(day23)]
fn input_generator(input: &str) -> HashMap<Coord, usize> {
    let mut map = HashMap::new();
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>,\sr=(\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (x, y, z): (isize, isize, isize) = (
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
        );
        map.insert(Coord(x, y, z), caps[4].parse::<usize>().unwrap());
    }
    map
}

#[aoc(day23, part1)]
fn solve_part1(map: &HashMap<Coord, usize>) -> usize {
    let (source_key, source_value) = map.iter().max_by_key(|&(_, v)| v).unwrap();
    let mut num_in_range = 0;
    for key in map.keys() {
        let &Coord(x, y, z) = key;
        let distance = ((x - source_key.0).abs()
            + (y - source_key.1).abs()
            + (z - source_key.2).abs()) as usize;
        if distance <= *source_value {
            num_in_range += 1;
        }
    }
    num_in_range
}

#[aoc(day23, part2)]
fn solve_part2(map: &HashMap<Coord, usize>) -> usize {
    let mut distances: BTreeMap<isize, isize> = BTreeMap::new();
    for (coord, signal_strength) in map {
        let &Coord(x, y, z) = coord;
        let distance = x + y + z;
        *distances
            .entry(distance - *signal_strength as isize)
            .or_default() += 1;
        *distances
            .entry(distance + *signal_strength as isize + 1)
            .or_default() -= 1;
    }

    let run: Vec<_> = distances
        .iter()
        .scan(0_isize, |state, (distance, count)| {
            *state += count;
            Some((distance, *state))
        })
        .collect();
    let max = run.iter().map(|&(_, count)| count).max().unwrap();
    let intervals: Vec<_> = run
        .iter()
        .zip(run.iter().skip(1))
        .filter_map(|(&(distance_a, count), &(distance_b, _))| {
            if count == max {
                Some((*distance_a, *distance_b - 1))
            } else {
                None
            }
        })
        .collect();
    if intervals.iter().any(|&(a, b)| a <= 0 && b >= 0) {
        0
    } else {
        intervals
            .iter()
            .map(|&(a, b)| if b < 0 { -b } else { a })
            .min()
            .unwrap() as usize
    }
}