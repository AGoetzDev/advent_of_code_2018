use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(char, char)> {
    input.lines().map(|l| {
        let split = l.split(" must be finished before step ").collect::<Vec<&str>>();
        let a = split[0].chars().last().unwrap();
        let b = split[1].chars().next().unwrap();
        (a, b)
    }).collect()
        
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[(char, char)]) -> String {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();
    let mut result: String = String::new();
    for (a, b) in input {
        map.entry(*a).or_insert(HashSet::new());
        let mut deps = map.entry(*b).or_insert(HashSet::new());
        deps.insert(*a);
    }
    let mut seen: HashSet<char> = HashSet::new();
    
    for i in 0..map.len() {
        let candidate = find_candidates(&seen, &map)[0];
        result.push(candidate);
        seen.insert(candidate);

    }
    result
        
}

fn find_candidates(seen: &HashSet<char>, dependencies: &HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut candidates: Vec<char> = dependencies.iter().filter(|(k,v)| (v.len() == 0 || seen.is_superset(v)) && !seen.contains(k)).map(|(k, _v)| *k).collect();
    candidates.sort();
    candidates
}

#[derive(Debug)]
struct Worker {
    task: Option<char>,
    time_finished: i32,
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[(char, char)]) -> i32 {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();
    let mut result: String = String::new();
    for (a, b) in input {
        map.entry(*a).or_insert(HashSet::new());
        let mut deps = map.entry(*b).or_insert(HashSet::new());
        deps.insert(*a);
    }
    let mut seen: HashSet<char> = HashSet::new();
    let mut workers: Vec<Worker> = vec!(Worker{task: None, time_finished: 0}, Worker{task: None, time_finished: 0}, 
                                   Worker{task: None, time_finished: 0}, Worker{task: None, time_finished: 0}, 
                                    Worker{task: None, time_finished: 0});
    let mut current_time = 0;
    let mut in_progress: HashSet<char> = HashSet::new();
    
    loop {
        //Check finished workers
        let mut finished: Vec<char> = Vec::new();
        for w in workers.iter_mut().filter(|w| w.time_finished == current_time) {
            if let Some(t) = w.task{
                    finished.push(t);
                    w.task = None;
            }
        } 

        //sort finished and insert to see
        finished.sort();
        for c in finished {
            seen.insert(c);
            result.push(c);
        }
        if result.len() == map.len() {
            break;
        }
        //get candidates
        let candidates = find_candidates(&seen, &map);
        //assign candidates to workers
        for c in candidates.iter() {
            for w in workers.iter_mut().filter(|w| w.task == None) {
                if !in_progress.contains(&c){
                    w.task = Some(*c);
                    in_progress.insert(*c);
                    let time = (*c as i32 - 4);
                    w.time_finished = current_time + time;
                }
            }
        }
        //find out which worker finishes first and advance that much time
        let min_worker = workers.iter().filter(|w| {
            match w.task {
                Some(i) => true,
                None => false
            }
        }).min_by_key(|w| w.time_finished);
        
        if let Some(w) = min_worker {
            current_time = w.time_finished;
        } else {
            break;
        }
        
    }
    println!("{}", result);
    current_time
        
}