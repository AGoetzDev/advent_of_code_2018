use std::collections::HashMap;
use chrono::prelude::*;

enum ParseState {
        Date,
        Id,
        State,
        None,
}
#[derive(Debug)]
enum GuardState {
        Start,
        Sleep,
        Awake
}
#[derive(Debug)]
pub struct GuardEvent {
        state: GuardState,
        date: NaiveDateTime,
        id: Option<i32>,
}

fn parse_event(input: &str)  -> GuardEvent{
        let mut id = Option::None;
        let mut guard_state = GuardState::Start;
        let mut state: ParseState = ParseState::None;
        let mut string_id: String = String::new();
        let mut string_date: String = String::new();

        for c in input.chars(){
                match state {
                        ParseState::Id => {
                                if c != ' ' {
                                        string_id.push(c);
                                } else {
                                        state = ParseState::None;   
                                }
                        },
                        ParseState::Date => {
                                if c != ']' {
                                        string_date.push(c);
                                } else {
                                        state = ParseState::State;   
                                }
                        },
                        ParseState::State => {
                                if c == 'G' {
                                        guard_state = GuardState::Start;
                                        state = ParseState::None; 
                                } else if c == 'w' {
                                        guard_state = GuardState::Awake; 
                                        state = ParseState::None;  
                                } else if c == 'f' {
                                        guard_state = GuardState::Sleep; 
                                        state = ParseState::None;  
                                }
                        },
                        ParseState::None => {
                                if c == '#' {
                                        state = ParseState::Id;
                                } else if c == '[' {
                                        state = ParseState::Date;
                                }
                        }       
                }
        }
        if string_id.len() > 0 {
                id = Some(string_id.parse::<i32>().unwrap());
        }
        GuardEvent {
                id: id,
                date: NaiveDateTime::parse_from_str(&string_date, "%Y-%m-%d %H:%M").unwrap(),
                state: guard_state,
        }
           
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<GuardEvent> {
   
 
   let mut parsed_input: Vec<GuardEvent> = input.lines().map( |s| {
                parse_event(s)
   }).collect();
   parsed_input.sort_by_key(|x| x.date);
   parsed_input
        
}


#[aoc(day4, part1)]
pub fn solve_part1(input: &[GuardEvent]) -> i32 {
        let mut map_total: HashMap<i32, i32> = HashMap::new();
        let mut map_minutes: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let mut current_id = 0;
        let mut current_minute = 0;
        for event in input { 
                match &event.state {
                        GuardState::Start => {
                                current_id = event.id.unwrap_or_else(|| current_id);
                        },
                        GuardState::Sleep => {
                                current_minute = event.date.minute();
                        },
                        GuardState::Awake => {
                                let total_asleep = map_total.entry(current_id).or_insert(0);
                                *total_asleep += (event.date.minute()  as i32 - current_minute  as i32);
                                let m_map = map_minutes.entry(current_id).or_insert(HashMap::new());
                                for i in current_minute..(event.date.minute()) {
                                        let mut count = m_map.entry(i as i32).or_insert(0);
                                        *count+=1;
                                }
                        },
                }
        }
        let mut max_id = 0;
        let mut max_count = 0;
        for (k, v) in map_total {
                if v > max_count {
                        max_count = v;
                        max_id = k;
                }
        };
        let mut max_minute = -1;
        let mut max_minute_count = 0;
        if let Some(m_map) = map_minutes.get(&max_id){
                for (k, v) in m_map {
                        if v > &max_minute_count {
                                max_minute_count = *v;
                                max_minute = *k;
                        }
                };
        };
        println!("{} {}", max_id, max_minute);
        max_id * max_minute
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[GuardEvent]) -> i32 {
        let mut map_minutes: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let mut current_id = 0;
        let mut current_minute = 0;
        for event in input { 
                match &event.state {
                        GuardState::Start => {
                                current_id = event.id.unwrap_or_else(|| current_id);
                        },
                        GuardState::Sleep => {
                                current_minute = event.date.minute();
                        },
                        GuardState::Awake => {
                                let m_map = map_minutes.entry(current_id).or_insert(HashMap::new());
                                for i in current_minute..(event.date.minute()) {
                                        let mut count = m_map.entry(i as i32).or_insert(0);
                                        *count+=1;
                                }
                        },
                }
        }
        let mut max_id = 0;
        let mut max_minute = -1;
        let mut max_minute_count = 0;

        let mut local_max_minute;
        let mut local_max_count;
        for (guard_id, map) in map_minutes {
                local_max_minute = 0;
                local_max_count = 0;
                for (minute, minute_count) in map {
                        if minute_count > local_max_count{
                                local_max_count = minute_count;
                                local_max_minute = minute;
                        }
                }
                if local_max_count > max_minute_count {
                        max_id = guard_id;
                        max_minute = local_max_minute;
                        max_minute_count = local_max_count;
                }
               
        };
       
        println!("{} {}", max_id, max_minute);
        max_id * max_minute
}


