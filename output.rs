pub mod arena {
    pub struct Arena {
        pub map: Vec<u32>,
        pub dim_x: usize,
        pub dim_y: usize,
    }
    impl Arena {
        pub fn new(x: usize, y: usize) -> Self {
            Arena {
                map: vec![0; x * y],
                dim_x: x,
                dim_y: y,
            }
        }
        pub fn is_mine(&self, x: usize, y: usize) -> bool {
            if self.map[y * self.dim_x + x] & 64 == 64 {
                return true;
            }
            false
        }
        pub fn is_enemy(&self, x: usize, y: usize) -> bool {
            if self.map[y * self.dim_x + x] & 32 == 32 {
                return true;
            }
            false
        }
        pub fn is_my_cel_next_to(&self, x: usize, y: usize) -> (bool, usize, usize) {
            if x > 0 && self.is_mine(x - 1, y) {
                return (true, x - 1, y);
            } else if self.is_mine(x + 1, y) {
                return (true, x + 1, y);
            } else if self.is_mine(x, y + 1) {
                return (true, x, y + 1);
            } else if y > 0 && self.is_mine(x, y - 1) {
                return (true, x, y - 1);
            }
            (false, 0, 0)
        }
        pub fn find_where_grow(&self) -> (bool, usize, usize, u32) {
            for y in 0..self.dim_y {
                for x in 0..self.dim_x {
                    if self.is_mine(x, y) == false {
                        let (is_my_cel, next_x, next_y) = self.is_my_cel_next_to(x, y);
                        if is_my_cel {
                            return (true, x, y, 0);
                        }
                    }
                }
            }
            (false, 0, 0, 0)
        }
        pub fn can_be_won_over(&self, x: usize, y: usize) -> bool {
            let mut elem = self.map[y * self.dim_x + x];
            elem = elem << 27;
            elem = elem >> 27;
            if elem == 0 || (7 <= elem && elem <= 11) {
                return true;
            }
            false
        }
        pub fn is_an_organ(&self, x: usize, y: usize) -> bool {
            let mut elem = self.map[y * self.dim_x + x];
            elem = elem << 27;
            elem = elem >> 27;
            if 2 <= elem && elem <= 6 {
                return true;
            }
            false
        }
    }
}
pub mod player {
    pub struct Player {
        pub a: usize,
        pub b: usize,
        pub c: usize,
        pub d: usize,
    }
    impl Player {
        pub fn new() -> Self {
            Player {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            }
        }
        pub fn update_values(&mut self, a: usize, b: usize, c: usize, d: usize) {
            self.a = a;
            self.b = b;
            self.c = c;
            self.d = d;
        }
    }
}
use arena::Arena;
use player::Player;
use std::io;
pub const WALL: u32 = 1;
pub const ROOT: u32 = 2;
pub const BASIC: u32 = 3;
pub const TENTACLE: u32 = 4;
pub const HARVESTER: u32 = 5;
pub const SPORER: u32 = 6;
pub const A: u32 = 7;
pub const B: u32 = 8;
pub const C: u32 = 9;
pub const D: u32 = 10;
pub const UNKNOWN: u32 = 11;
macro_rules ! parse_input { ( $ x : expr , $ t : ident ) => { $ x . trim ( ) . parse ::<$ t > ( ) . unwrap ( ) } ; }
fn main() {
    let mut input_line: String = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let cols = parse_input!(inputs[0], usize);
    let rows = parse_input!(inputs[1], usize);
    let mut arena = Arena::new(rows, cols);
    let mut guapo = Player::new();
    let mut opponent = Player::new();
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32);
        for _ in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], usize);
            let y = parse_input!(inputs[1], usize);
            let mut new_elem: u32;
            let _type = inputs[2].trim().to_string();
            match _type.as_str() {
                "WALL" => new_elem = WALL,
                "ROOT" => new_elem = ROOT,
                "BASIC" => new_elem = BASIC,
                "TENTACLE" => new_elem = TENTACLE,
                "HARVESTER" => new_elem = HARVESTER,
                "SPORER" => new_elem = SPORER,
                "A" => new_elem = A,
                "B" => new_elem = B,
                "C" => new_elem = C,
                "D" => new_elem = D,
                _ => new_elem = UNKNOWN,
            }
            let owner = parse_input!(inputs[3], i32);
            let new_owner: u32;
            if owner == 1 {
                new_owner = 1;
            } else if owner == 0 {
                new_owner = 0;
            } else {
                new_owner = 42
            }
            if owner >= 0 {
                new_elem += 32 * (new_owner + 1);
            }
            let mut organ_id = parse_input!(inputs[4], u32);
            if organ_id > 0 {
                organ_id = organ_id << 16;
                new_elem += organ_id;
            }
            let index = cols * y + x;
            arena.map[index as usize] = new_elem;
            let _organ_dir = inputs[5].trim().to_string();
            let _organ_parent_id = parse_input!(inputs[6], u32);
            let _organ_root_id = parse_input!(inputs[7], u32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_a = parse_input!(inputs[0], usize);
        let my_b = parse_input!(inputs[1], usize);
        let my_c = parse_input!(inputs[2], usize);
        let my_d = parse_input!(inputs[3], usize);
        guapo.update_values(my_a, my_b, my_c, my_d);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opp_a = parse_input!(inputs[0], usize);
        let opp_b = parse_input!(inputs[1], usize);
        let opp_c = parse_input!(inputs[2], usize);
        let opp_d = parse_input!(inputs[3], usize);
        opponent.update_values(opp_a, opp_b, opp_c, opp_d);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let required_actions_count = parse_input!(input_line, i32);
        for _ in 0..required_actions_count as usize {
            let mut output = String::new();
            let (to_build, x_new, y_new, magic) = arena.find_where_grow();
            if to_build {
                output.push_str("GROW ");
                output.push_str("1");
                output.push_str(" ");
                output.push_str(&x_new.to_string());
                output.push_str(" ");
                output.push_str(&y_new.to_string());
                output.push_str(" BASIC");
                println!("{}", output);
            } else {
                println!("WAIT");
            }
        }
    }
}

