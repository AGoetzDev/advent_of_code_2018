use std::{
    fmt::{self, Display},
    iter::FromIterator,
};

fn fmt_pot(pot: bool) -> char {
    if pot {
        '#'
    } else {
        '.'
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    input: Vec<bool>,
    output: bool,
}

impl Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut t = String::with_capacity(self.input.len() + 5);
        self.input.iter().for_each(|&pot| {
            t.push(fmt_pot(pot));
        });
        t.push_str(" => ");
        t.push(fmt_pot(self.output));
        f.write_str(&t)
    }
}

impl Transition {
    pub fn new(input: Vec<bool>, output: bool) -> Self {
        Transition { input, output }
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Plantation {
    pots: Vec<bool>,
    transitions: Vec<Transition>,
}

impl AsRef<Plantation> for Plantation {
    fn as_ref(&self) -> &Plantation {
        self
    }
}

impl Plantation {
    pub fn new(
        init_state: impl IntoIterator<Item = bool>,
        transitions: Vec<Transition>,
    ) -> Self {
        Self {
            pots: Vec::from_iter(init_state.into_iter()),
            transitions: transitions,
        }
    }
 
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Plantation {
    let mut lines = input.trim().lines();

    let init_state = lines.next().unwrap().trim()[15..]
        .chars()
        .map(|chr| chr == '#');

    let _ = lines.next(); // skip one line

    let transitions: Vec<Transition> = lines.map(|line| {
        let mut split = line.split("=>");
        let input_t = split
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|chr| chr == '#')
            .collect();
        let output = split
            .next()
            .unwrap()
            .trim()
            .chars()
            .next()
            .map(|chr| chr == '#')
            .unwrap();
        Transition::new(input_t, output)
    }).collect();

    Plantation::new(init_state, transitions)
}

#[aoc(day12, part1)]
pub fn  solve_part1(plantation: &Plantation) -> i64 {
    let (future_plantation, offset) = advance(plantation, 20);
    sum(&future_plantation, offset)
}

fn sum(plantation: &Plantation, offset: i64) -> i64 {
   
    plantation.pots.iter()
        .zip(offset.. plantation.pots.len() as i64 + offset)
        .fold(0, |acc, (&pot, num)| if pot { acc + num } else { acc })
}

fn advance(plantation: &Plantation, steps: u64) -> (Plantation, i64) {
    let transitions = plantation.transitions.clone();
    let input_size = transitions[0].input.len();
    let center =input_size / 2;
    let outer = &vec![false; input_size][..];

    let mut offset = 0;
    let mut pots = plantation.pots.clone();

    while &pots[pots.len() - input_size..] != outer {
        pots.push(false);
    }
    while &pots[0..input_size] != outer {
        pots.insert(0, false);
        offset -= 1;
    }

    for gen_num in 0..steps {
        let mut next_pots = pots.clone();

        next_pots
            .iter_mut()
            .skip(center)
            .zip(pots.windows(input_size))
            .for_each(|(center_pot, pots)| {
                *center_pot = transitions
                    .iter()
                    .find_map(|transition| {
                        if transition.input == pots {
                            Some(transition.output)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);
            });

        let mut delta = 0;
        while &next_pots[0..input_size] == outer {
            next_pots.remove(0);
            delta += 1;
        }
        while &next_pots[0..input_size] != outer {
            next_pots.insert(0, false);
            delta -= 1;
        }
        offset += delta;
        while &next_pots[next_pots.len() - input_size..] != outer {
            next_pots.push(false);
        }

        if next_pots == pots {
            offset += (steps as i64 - gen_num as i64 - 1) * delta;
            break;
        }
        pots = next_pots;

    }
    (
        Plantation {
            pots,
            transitions,
        },
        offset,
    )
}

#[aoc(day12, part2)]
pub fn  solve_part2(plantation: &Plantation) -> i64 {
    let (future_plantation, offset) = advance(plantation, 50_000_000_000);
    sum(&future_plantation, offset)
}

