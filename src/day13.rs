use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FieldType {
    StraightV,
    StraightH,
    Intersection,
    TurnL,
    TurnR,
    None
}
#[derive(Debug, Clone)]
pub struct Field {
    ftype: FieldType,
    cart: i32

}
impl Field {
    fn clear(&mut self) {
        self.cart = 0;
    }
    fn fmt(&self) -> char {
        match self.cart {
            0 => {
                match self.ftype {
                    FieldType::StraightV => '|',
                    FieldType::StraightH => '-',
                    FieldType::Intersection => '+',
                    FieldType::TurnL => '\\',
                    FieldType::TurnR => '/',
                    FieldType::None => ' ',
                }
                
            },
            _ => 'c'
        }
    }
}

impl AsRef<Field> for Field {
    fn as_ref(&self) -> &Field {
        self
    }
}
#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}
#[derive(Debug, Clone)]
struct Cart {
    id: i32,
    dir: Direction,
    cur_int: u32,
    x: usize,
    y: usize,
}

impl AsRef<Cart> for Cart {
    fn as_ref(&self) -> &Cart {
        self
    }
}

impl Cart {
    fn  drive(&mut self) {
        match &self.dir {
            Direction::Up => self.y-=1,
            Direction::Down => self.y+=1,
            Direction::Left => self.x-=1,
            Direction::Right => self.x+=1
        }
    }
   
    fn process_field(&mut self, ftype: FieldType) {
        match ftype {
            FieldType::Intersection => match self.cur_int {
                0 => {
                    self.dir = self.dir.left();
                    self.cur_int = 1;
                },
                1 => {
                    self.cur_int = 2;
                },
                2 => {
                     self.dir = self.dir.right();
                     self.cur_int = 0;
                },
                _ => {}
            },
            FieldType::TurnL => match self.dir {
                Direction::Up => self.dir = self.dir.left(),
                Direction::Down => self.dir = self.dir.left(),
                Direction::Left=> self.dir = self.dir.right(),
                Direction::Right => self.dir = self.dir.right(),
                
            },
            FieldType::TurnR => match self.dir {
                Direction::Up => self.dir = self.dir.right(),
                Direction::Down => self.dir = self.dir.right(),
                Direction::Left=> self.dir = self.dir.left(),
                Direction::Right => self.dir = self.dir.left(),
                
            },
            _=> {}
            
        }
    }
}

pub struct Board {
    fields: Vec<Vec<Field>>,
    carts: Vec<Cart>,
}
impl AsRef<Board> for Board {
    fn as_ref(&self) -> &Board {
        self
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Board {
    let mut fields: Vec<Vec<Field>> = Vec::new();
    let mut cars: Vec<Cart> = Vec::new();
    let mut cart_id = 0;
    for (y, l) in input.lines().enumerate() {
        let mut fields_row: Vec<Field> = Vec::new();
        for (x, c) in l.chars().enumerate() {
            match c {
                '|' => fields_row.push(Field {ftype: FieldType::StraightV, cart: 0}),
                '-' => fields_row.push(Field {ftype: FieldType::StraightH, cart: 0}),
                '+' => fields_row.push(Field {ftype: FieldType::Intersection, cart: 0}),
                '/' => fields_row.push(Field {ftype: FieldType::TurnR, cart: 0}),
                '\\' => fields_row.push(Field {ftype: FieldType::TurnL, cart: 0}),
                '^' =>  {
                    cart_id+=1;
                    cars.push(Cart {id: cart_id, dir: Direction::Up, cur_int: 0, x: x, y: y});
                    fields_row.push(Field {ftype: FieldType::StraightV, cart: cart_id});
                },
                '<' =>  {
                    cart_id+=1;
                    cars.push(Cart {id: cart_id, dir: Direction::Left, cur_int: 0, x: x, y: y});
                    fields_row.push(Field {ftype: FieldType::StraightH, cart: cart_id});
                },
                '>' =>  {
                    cart_id+=1;
                    cars.push(Cart {id: cart_id, dir: Direction::Right, cur_int: 0, x: x, y: y});
                    fields_row.push(Field {ftype: FieldType::StraightH, cart: cart_id});
                },
                'v' =>  {
                    cart_id+=1;
                    cars.push(Cart {id: cart_id, dir: Direction::Down, cur_int: 0, x: x, y: y});
                    fields_row.push(Field {ftype: FieldType::StraightV, cart: cart_id});
                }
                _ => fields_row.push(Field {ftype: FieldType::None, cart: 0})
            }
        }
        fields.push(fields_row);
    }
    Board {fields: fields, carts: cars}
    
}

#[aoc(day13, part1)]
pub fn  solve_part1(board: &Board) -> usize {
    let mut fields = board.fields.clone();
    let mut carts = board.carts.clone();
    let mut cart_lookup: HashMap<i32, bool> = HashMap::new();
    for cart in carts.iter() {
        cart_lookup.entry(cart.id).or_insert(true);
    }
    
    loop {

        carts.sort_by_key(|cart| (cart.y, cart.x));
        for cart in carts.iter_mut() {
            if *cart_lookup.entry(cart.id).or_insert(false) {
                let prev_x = cart.x;
                let prev_y = cart.y;
                cart.drive();
                fields[prev_y][prev_x].clear();
                if fields[cart.y][cart.x].cart != 0 {
                    //collision
                    *cart_lookup.entry(fields[cart.y][cart.x].cart).or_insert(false) = false;
                    *cart_lookup.entry(cart.id).or_insert(false) = false;
                    fields[cart.y][cart.x].clear();
                    println!("{},{}", cart.x, cart.y);
                    return 0;
                } else {
                    fields[cart.y][cart.x].cart = cart.id;
                    let ftype = fields[cart.y][cart.x].ftype.clone();
                    cart.process_field(ftype);
                }
            }
            
            
        }
        /*for l in fields.iter() {
            let mut output = String::new();
            for f in l.iter() {
                let c = f.fmt();
                output.push(c);
            }
            println!("{}", output);
        }*/
    }
}

#[aoc(day13, part2)]
pub fn  solve_part2(board: &Board) -> usize {
    let mut fields = board.fields.clone();
    let mut carts = board.carts.clone();
    let mut cart_lookup: HashMap<i32, bool> = HashMap::new();
    for cart in carts.iter() {
        cart_lookup.entry(cart.id).or_insert(true);
    }
    
    loop {
        if carts.len() == 1{
            println!("{},{}", carts[0].x, carts[0].y);
           return 1;
        }
        carts.sort_by_key(|cart| (cart.y, cart.x));
        for cart in carts.iter_mut() {
            if *cart_lookup.entry(cart.id).or_insert(false) {
                let prev_x = cart.x;
                let prev_y = cart.y;
                cart.drive();
                fields[prev_y][prev_x].clear();
                if fields[cart.y][cart.x].cart != 0 {
                    //collision
                    *cart_lookup.entry(fields[cart.y][cart.x].cart).or_insert(false) = false;
                    *cart_lookup.entry(cart.id).or_insert(false) = false;
                    fields[cart.y][cart.x].clear();
                    
                } else {
                    fields[cart.y][cart.x].cart = cart.id;
                    let ftype = fields[cart.y][cart.x].ftype.clone();
                    cart.process_field(ftype);
                }
                
            }
            
            
        }
        carts.retain(|cart| *cart_lookup.entry(cart.id).or_insert(false));
    }
}