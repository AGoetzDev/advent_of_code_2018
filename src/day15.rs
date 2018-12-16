use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Field {
    //-1 for rock, 0 for open, >1 for actor id
    content: i32,
    x: usize,
    y: usize

}

impl Field {
    fn set_content(&mut self, new_content: i32){
        self.content = new_content;
    }
    fn fmt(&self) -> char {
        match self.content {
            0 => '.',
            -1 => '#',
            _ => 'c'
        }
    }

}

impl AsRef<Field> for Field {
    fn as_ref(&self) -> &Field {
        self
    }
}


#[derive(Debug, Clone, PartialEq)]
enum ActorType {
    Elf,
    Gobbo
}

#[derive(Debug, Clone)]
pub struct Board {
    actors: HashMap<i32, Actor>,
    fields: Vec<Vec<Field>>,
}
impl Board {
   
    fn damage_actor(&mut self, actor_id: i32 , damage: i32){
        if let Some(a) = self.actors.get_mut(&actor_id) {
            let actor = a;
            actor.hp -= damage;
            if actor.hp <= 0 {
                self.fields[actor.y][actor.x].set_content(0);
            }
        } else {
            println!("Error! Invalid actor id in damage_actor: {}", actor_id);
        }
        
    }
    fn move_actor(&mut self, actor_id: i32, x:usize, y: usize){
        if let Some(a) = self.actors.get_mut(&actor_id) {
            let actor = a;
            self.fields[actor.y][actor.x].set_content(0);
            actor.x = x;
            actor.y = y;
            self.fields[actor.y][actor.x].set_content(actor.id);
        } else {
            println!("Error! Invalid actor id in move_actor: {}", actor_id);
        }
        
    }

    fn check_move(&self, actor_id: i32) -> Option<(usize, usize)>{
        
        if let Some(a) = self.actors.get(&actor_id) {
            let cur_actor = a;
            let mut paths: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
            let mut cur_x = cur_actor.x;
            let mut cur_y = cur_actor.y ;
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut to_visit: Vec<(usize, usize)> = Vec::new();
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut enemy_found: bool = false;
            queue.push_back((cur_x, cur_y));
            loop {
                while queue.len() != 0 {
                    if let Some(t) = queue.pop_front() {
                        //println!("Checking position for actor {}: {},{}", actor_id, t.0, t.1);
                        let current_t: (usize, usize) = t;
                        cur_x = current_t.0;
                        cur_y = current_t.1;
                        visited.insert(current_t);
                        let field = &self.fields[current_t.1][current_t.0];
                        if field.content > 0 && cur_actor.id != field.content {
                            //check if its an enemy
                            if let Some(a) =  self.actors.get(&field.content){
                                let actor: &Actor = a;
                                if cur_actor.atype != actor.atype {
                                    enemy_found = true;
                                    break;
                                }
                            } else {
                                println!("Error: Invalid actor in check move get enemy: {}", field.content);
                                return None;
                            }
                            
                        } else if field.content == 0 || cur_actor.id == field.content {
                            //path we can move to, check adjacent in read order
                            let y_max = self.fields.len();
                            let x_max = self.fields[0].len();
                            //up
                            let mut t = (cur_x, cur_y-1);
                            if cur_y as i32 - 1 >= 0  && !visited.contains(&t){
                                paths.insert(t, (cur_x, cur_y));
                                to_visit.push(t);
                                visited.insert(t);
                            }
                            //left
                            t = (cur_x-1, cur_y);
                            if cur_x as i32 - 1 >= 0  && !visited.contains(&t){
                                paths.insert(t, (cur_x, cur_y));
                                to_visit.push(t);
                                visited.insert(t);
                            }
                            //right
                            t = (cur_x+1, cur_y);
                            if cur_x + 1 < x_max  && !visited.contains(&t){
                                paths.insert(t, (cur_x, cur_y));
                                to_visit.push(t);
                                visited.insert(t);
                            }
                            //down
                            t = (cur_x, cur_y+1);
                            if cur_y+1 < y_max  && !visited.contains(&t){
                                paths.insert(t, (cur_x, cur_y));
                                to_visit.push(t);
                                visited.insert(t);
                            }
                        }
                    } else {
                        println!("Error: nothing to pop in queue");
                        return None;
                    }
                    
                }
                if enemy_found {
                    let mut pos = (cur_x, cur_y);
                    loop {
                        match paths.get(&pos) {
                            Some(t) => {
                                if *t == (cur_actor.x, cur_actor.y) {
                                    return Some((pos.0, pos.1));
                                }
                                pos = *t;
                            },
                            None => {
                                //found first tile on path to enemy, do actual move
                                return Some((pos.0, pos.1));
                            }
                        }
                    }
                } else if to_visit.len() > 0 {
                    for t in to_visit.iter() {
                        queue.push_back(*t);
                    }
                    to_visit.clear();
                } else {
                    break;
                }
            }
            return None;
        } else {
            println!("Error: invalid actor id in check move");
            return None;
        }
        
        
    }

    fn check_attack(&self, actor_id: i32) -> Option<i32>{
        if let Some(a) = self.actors.get(&actor_id){
            let cur_actor = a;
            let y_max = self.fields.len();
            let x_max = self.fields[0].len();
            let mut enemy_id = -1;
            let mut hp = std::i32::MAX;
            //up
            let mut t = (cur_actor.x, cur_actor.y-1);
            if cur_actor.y as i32 - 1 >= 0 && self.fields[t.1][t.0].content > 0{
                if let Some(a) =  self.actors.get(&self.fields[t.1][t.0].content){
                    let actor: &Actor = a;
                    if cur_actor.atype != actor.atype  && actor.hp < hp{
                        enemy_id = actor.id;
                        hp = actor.hp;
                    }
                 } else {
                    println!("Error: Invalid actor in check attack 1: {},{} id: {}",t.1, t.0, self.fields[t.1][t.0].content);
                    return None;
                }
                
            }
            //left
            t = (cur_actor.x-1, cur_actor.y);
            if cur_actor.x as i32 - 1 >= 0 && self.fields[t.1][t.0].content > 0{
                if let Some(a) =  self.actors.get(&self.fields[t.1][t.0].content){
                    let actor: &Actor = a;
                    if cur_actor.atype != actor.atype  && actor.hp < hp{
                        enemy_id = actor.id;
                        hp = actor.hp;
                    }
                 } else {
                    println!("Error: Invalid actor in check attack 2: {},{} id: {}",t.1, t.0, self.fields[t.1][t.0].content);
                    return None;
                }
            }
            //right
            t = (cur_actor.x+1, cur_actor.y);
            if cur_actor.x + 1 < x_max && self.fields[t.1][t.0].content > 0{
               if let Some(a) =  self.actors.get(&self.fields[t.1][t.0].content){
                    let actor: &Actor = a;
                    if cur_actor.atype != actor.atype  && actor.hp < hp{
                        enemy_id = actor.id;
                        hp = actor.hp;
                    }
                 } else {
                    println!("Error: Invalid actor in check attack 3: {},{} id: {}",t.1, t.0, self.fields[t.1][t.0].content);
                    return None;
                }
            }
            //down
            t = (cur_actor.x, cur_actor.y+1);
            if cur_actor.y+1 < y_max && self.fields[t.1][t.0].content > 0{
                if let Some(a) =  self.actors.get(&self.fields[t.1][t.0].content){
                    let actor: &Actor = a;
                    if cur_actor.atype != actor.atype  && actor.hp < hp{
                        enemy_id = actor.id;
                    }
                 } else {
                    println!("Error: Invalid actor in check attack 4: {},{} id: {}",t.1, t.0, self.fields[t.1][t.0].content);
                    return None;
                }
            }
            if enemy_id > -1 {
                Some(enemy_id)
            } else {
                None
            }
        } else {
            println!("Error: invalid actor id in check attack");
            None
        }
        
    }

    fn check_dead_elf(&self, actor_id: i32) -> bool {
         if let Some(a) = self.actors.get(&actor_id){
             return a.hp <= 0 && a.atype == ActorType::Elf
         } else {
             println!("Error! Invalid actor id in check_dead_elf: {}", actor_id); 
             false
         }
    }
    


}

impl AsRef<Board> for Board {
    fn as_ref(&self) -> &Board {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Actor {
    id: i32,
    atype: ActorType,
    power: i32,
    hp: i32,
    x: usize,
    y: usize,
}

impl AsRef<Actor> for Actor {
    fn as_ref(&self) -> &Actor {
        self
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Board {
    let mut fields: Vec<Vec<Field>> = Vec::new();
    let mut actors: HashMap<i32, Actor> = HashMap::new();
    let mut actor_id = 0;
    for (y, l) in input.lines().enumerate() {
        let mut fields_row: Vec<Field> = Vec::new();
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => fields_row.push(Field {content: -1, x: x, y: y}),
                '.' => fields_row.push(Field {content: 0, x: x, y: y}),
                'E' => {
                    actor_id+=1;
                    let actor = Actor {id: actor_id, atype: ActorType::Elf, x: x, y: y, hp: 200, power: 3};
                   
                    actors.insert(actor_id, actor);
                    fields_row.push(Field {content: actor_id, x: x, y: y});
                },
                'G' => {
                    actor_id+=1;
                    let actor = Actor {id: actor_id, atype: ActorType::Gobbo, x: x, y: y, hp: 200, power: 3};
                    actors.insert(actor_id, actor);
                    fields_row.push(Field {content: actor_id, x: x, y: y});
                },
                _ => fields_row.push(Field {content: -1, x: x, y: y}),
            }
        }
        fields.push(fields_row);
    }
    Board {fields: fields, actors: actors}
    
}

#[aoc(day15, part1)]
pub fn  solve_part1(board: &Board) -> i32 {
    let mut board = board.clone();
    let mut not_finished = true;
    let mut rounds = 0;
  
    while not_finished {
        not_finished = false;
        //hacky borrow checker workaround
        let mut actors_ordered: Vec<Actor> = board.actors.iter().map( |e| e.1.clone()).collect();
        actors_ordered.sort_by_key(|actor| (actor.y, actor.x));
    
        for actor in actors_ordered.iter() {
            if board.actors.get(&actor.id).unwrap().hp > 0 {
                if let Some(id) =  board.check_attack(actor.id) {
                    board.damage_actor(id, 3);
                    not_finished = true;
                } else {
                    if let Some(t) = board.check_move(actor.id) {
                        board.move_actor(actor.id, t.0, t.1);
                        not_finished = true;
                        if let Some(id) =  board.check_attack(actor.id) {
                            board.damage_actor(id, 3);
                            
                        }
                    } else {
                        //println!("No move found for: {}", actor.id);
                    }
                }
            }
        }
        rounds+=1;
    }
    let sum_hp: i32 = board.actors.iter().filter(|e| e.1.hp > 0).map(|e| e.1.hp).sum();
   
   println!("rounds: {}", rounds-2);
   println!("hp: {}", sum_hp);
    /*for l in board.fields.iter() {
        let mut output = String::new();
        for f in l.iter() {
            let c = f.fmt();
            output.push(c);
        }
        println!("{}", output);
    }*/
    
    
        
    
    sum_hp*(rounds-2)
}

#[aoc(day15, part2)]
pub fn  solve_part2(board: &Board) -> i32 {
    let mut power = 3;
    loop {
        let mut board = board.clone();
        let mut not_finished = true;
        let mut rounds = 0;
        power +=1;
        let mut elves_alive = true;
        for (_k, v) in board.actors.iter_mut(){
            if v.atype == ActorType::Elf {
                v.power = power;
            }
        }
        while not_finished {
            not_finished = false;
            //hacky borrow checker workaround
            let mut actors_ordered: Vec<Actor> = board.actors.iter().map( |e| e.1.clone()).collect();
            actors_ordered.sort_by_key(|actor| (actor.y, actor.x));
        
            for actor in actors_ordered.iter() {
                if board.actors.get(&actor.id).unwrap().hp > 0 {
                    if let Some(id) =  board.check_attack(actor.id) {
                        board.damage_actor(id, actor.power);
                        not_finished = true;
                        if board.check_dead_elf(id) {
                            not_finished = false;
                            elves_alive = false;
                            break;
                        }
                    } else {
                        if let Some(t) = board.check_move(actor.id) {
                            board.move_actor(actor.id, t.0, t.1);
                            not_finished = true;
                            if let Some(id) =  board.check_attack(actor.id) {
                                board.damage_actor(id, actor.power);
                                if board.check_dead_elf(id) {
                                    not_finished = false;
                                    elves_alive = false;
                                    break;
                                }
                            }
                        } else {
                            //println!("No move found for: {}", actor.id);
                        }
                    }
                }
            }
            rounds+=1;
        }
        if elves_alive {
            let sum_hp: i32 = board.actors.iter().filter(|e| e.1.hp > 0).map(|e| e.1.hp).sum();
            println!("power: {}", power);
            println!("rounds: {}", rounds-2);
            println!("hp: {}", sum_hp);
            return sum_hp*(rounds-2)
        }

    }
   
    
}