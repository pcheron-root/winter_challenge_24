pub mod arena {
    pub struct Arena {
        pub map: Vec<u32>,
        pub nb_col: usize,
        pub nb_lin: usize,
    }
    impl Arena {
        pub fn new(rows: usize, cols: usize) -> Self {
            Arena {
                map: vec![0; cols * rows],
                nb_col: cols,
                nb_lin: rows,
            }
        }
        pub fn is_enemy_near(&self, id: u32) -> (bool, usize, usize, String, String) {
            let mut map = vec![4; self.nb_col * self.nb_lin];
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if is_wall(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 64;
                    }
                    if is_from_organ(self.map[y * self.nb_col + x], id) {
                        map[y * self.nb_col + x] = 0;
                    }
                }
            }
            for i in 0..2 {
                for y in 0..self.nb_lin {
                    for x in 0..self.nb_col {
                        if map[y * self.nb_col + x] == i {
                            eprintln!("je trouve un {}", i);
                            if map[y * self.nb_col + x + 1] > i + 1 &&
                                map[y * self.nb_col + x + 1] != 64
                            {
                                map[y * self.nb_col + x + 1] = i + 1;
                                if i == 1 && is_oppo(self.map[y * self.nb_col + x + 1]) {
                                    return (true, x, y, " TENTACLE".to_string(), " E".to_string());
                                }
                            }
                            if map[y * self.nb_col + x - 1] > i + 1 &&
                                map[y * self.nb_col + x - 1] != 64
                            {
                                map[y * self.nb_col + x - 1] = i + 1;
                                if i == 1 && is_oppo(self.map[y * self.nb_col + x - 1]) {
                                    return (true, x, y, " TENTACLE".to_string(), " W".to_string());
                                }
                            }
                            if map[(y + 1) * self.nb_col + x] > i + 1 &&
                                map[(y + 1) * self.nb_col + x] != 64
                            {
                                map[(y + 1) * self.nb_col + x] = i + 1;
                                if i == 1 && is_oppo(self.map[(y + 1) * self.nb_col + x]) {
                                    return (true, x, y, " TENTACLE".to_string(), " S".to_string());
                                }
                            }
                            if map[(y - 1) * self.nb_col + x] > i + 1 &&
                                map[(y - 1) * self.nb_col + x] != 64
                            {
                                map[(y - 1) * self.nb_col + x] = i + 1;
                                if i == 1 && is_oppo(self.map[(y - 1) * self.nb_col + x]) {
                                    return (true, x, y, " TENTACLE".to_string(), " N".to_string());
                                }
                            }
                        }
                    }
                }
            }
            return (false, 0, 0, "".to_string(), "".to_string());
        }
        pub fn next_move(&self, id: u32) -> (usize, usize, String, String) {
            print_root(self.map.clone(), self.nb_col, self.nb_lin);
            let (is_near, x, y, order, direction) = self.is_enemy_near(id);
            if is_near {
                eprint!("un enemy est proche");
                return (x, y, order, direction);
            }
            return self.looking_for_prot(id);
        }
        pub fn looking_for_prot(&self, id: u32) -> (usize, usize, String, String) {
            let mut map = vec![4; self.nb_col * self.nb_lin];
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if is_wall(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 64;
                    }
                    if is_mine(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 0;
                    }
                }
            }
            for i in 0..7 {
                for y in 0..self.nb_lin {
                    for x in 0..self.nb_col {
                        if map[y * self.nb_col + x] == i {
                            if map[y * self.nb_col + x + 1] > i + 1 &&
                                map[y * self.nb_col + x + 1] != 64
                            {
                                map[y * self.nb_col + x + 1] = i + 1;
                                if i == 1 && is_protein(self.map[y * self.nb_col + x + 1]) &&
                                    !self.is_ate(x + 1, y)
                                {
                                    return (x, y, " HARVESTER".to_string(), " E".to_string());
                                }
                            }
                            if map[y * self.nb_col + x - 1] > i + 1 &&
                                map[y * self.nb_col + x - 1] != 64
                            {
                                map[y * self.nb_col + x - 1] = i + 1;
                                if i == 1 && is_protein(self.map[y * self.nb_col + x - 1]) &&
                                    !self.is_ate(x - 1, y)
                                {
                                    return (x, y, " HARVESTER".to_string(), " W".to_string());
                                }
                            }
                            if map[(y + 1) * self.nb_col + x] > i + 1 &&
                                map[(y + 1) * self.nb_col + x] != 64
                            {
                                map[(y + 1) * self.nb_col + x] = i + 1;
                                if i == 1 && is_protein(self.map[(y + 1) * self.nb_col + x]) &&
                                    !self.is_ate(x, y + 1)
                                {
                                    return (x, y, " HARVESTER".to_string(), " S".to_string());
                                }
                            }
                            if map[(y - 1) * self.nb_col + x] > i + 1 &&
                                map[(y - 1) * self.nb_col + x] != 64
                            {
                                map[(y - 1) * self.nb_col + x] = i + 1;
                                if i == 1 && is_protein(self.map[(y + 1) * self.nb_col + x]) &&
                                    !self.is_ate(x, y + 1)
                                {
                                    return (x, y, " HARVESTER".to_string(), " N".to_string());
                                }
                            }
                            if is_protein(self.map[y * self.nb_col + x]) && !self.is_ate(x, y) {
                                eprint!("pas encore mange");
                                return (x, y, " BASIC".to_string(), "".to_string());
                            }
                        }
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 1 && !self.is_ate(x, y) {
                        return (x, y, " BASIC".to_string(), "".to_string());
                    }
                }
            }
            return (0, 0, " BASIC".to_string(), "".to_string());
        }
        pub fn is_ate(&self, x: usize, y: usize) -> bool {
            if is_mouth(self.map[(y - 1) * self.nb_col + x]) &&
                is_south(self.map[(y - 1) * self.nb_col + x])
            {
                return true;
            } else if is_mouth(self.map[(y + 1) * self.nb_col + x]) &&
                       is_north(self.map[(y + 1) * self.nb_col + x])
            {
                return true;
            } else if is_mouth(self.map[y * self.nb_col + x - 1]) &&
                       is_east(self.map[y * self.nb_col + x - 1])
            {
                return true;
            } else if is_mouth(self.map[y * self.nb_col + x + 1]) &&
                       is_west(self.map[y * self.nb_col + x + 1])
            {
                return true;
            }
            false
        }
    }
    pub fn is_mine(x: u32) -> bool {
        if x & 64 == 64 {
            return true;
        }
        false
    }
    pub fn is_oppo(x: u32) -> bool {
        if x & 32 == 32 {
            return true;
        }
        false
    }
    pub fn is_wall(mut x: u32) -> bool {
        x = x << 27;
        x = x >> 27;
        if x == 1 {
            return true;
        }
        false
    }
    pub fn is_from_organ(x: u32, id: u32) -> bool {
        id == (x >> 16)
    }
    pub fn is_protein(mut x: u32) -> bool {
        x = x << 27;
        x = x >> 27;
        if 6 < x && x < 11 {
            return true;
        }
        false
    }
    pub fn print_map(map: Vec<u32>, slice: usize, nb_slice: usize) {
        eprintln!("print map\n");
        for i in 0..nb_slice {
            eprintln!("{:?}\n", &map[(i * slice)..(slice * (1 + i))]);
        }
    }
    pub fn print_enemies(mut map: Vec<u32>, slice: usize, nb_slice: usize) {
        eprintln!("print enemies\n");
        for j in 0..(slice * nb_slice) {
            if is_oppo(map[j]) {
                map[j] = 32;
            }
            if is_mine(map[j]) {
                map[j] = 64;
            }
        }
        for i in 0..nb_slice {
            eprintln!("{:?}\n", &map[(i * slice)..(slice * (1 + i))]);
        }
    }
    pub fn print_root(mut map: Vec<u32>, slice: usize, nb_slice: usize) {
        eprintln!("print enemies\n");
        for j in 0..(slice * nb_slice) {
            map[j] = map[j] >> 16;
        }
        for i in 0..nb_slice {
            eprintln!("{:?}\n", &map[(i * slice)..(slice * (1 + i))]);
        }
    }
    pub fn is_north(x: u32) -> bool {
        if (x & 128) == 0 && (x & 256) == 0 {
            return true;
        }
        false
    }
    pub fn is_south(x: u32) -> bool {
        if (x & 128) == 128 && (x & 256) == 0 {
            return true;
        }
        false
    }
    pub fn is_west(x: u32) -> bool {
        if (x & 128) == 0 && (x & 256) == 256 {
            return true;
        }
        false
    }
    pub fn is_east(x: u32) -> bool {
        if (x & 128) == 128 && (x & 256) == 256 {
            return true;
        }
        false
    }
    pub fn is_mouth(x: u32) -> bool {
        if (x & 5) == 5 {
            return true;
        }
        false
    }
    #[test]
    fn test_is_mine() {
        let mut x: u32 = 3;
        x += 64;
        assert_eq!(is_mine(x), true);
        let mut y: u32 = 8;
        y += 64;
        assert_eq!(is_mine(y), true);
        let mut z: u32 = 8;
        z += 16;
        assert_eq!(is_mine(z), false);
    }
    #[test]
    fn test_is_oppo() {
        let mut x: u32 = 3;
        x += 32;
        assert_eq!(is_oppo(x), true);
        let mut y: u32 = 8;
        y += 32;
        assert_eq!(is_oppo(y), true);
        let mut z: u32 = 8;
        z += 64;
        assert_eq!(is_oppo(z), false);
    }
    #[test]
    fn test_is_wall() {
        let mut x: u32 = 1;
        x += 64;
        assert_eq!(is_wall(x), true);
        let y: u32 = 1;
        assert_eq!(is_wall(y), true);
        let mut z: u32 = 4;
        z += 64;
        assert_eq!(is_wall(z), false);
    }
    #[test]
    fn test_is_protein() {
        let mut x: u32 = 7;
        x += 64;
        assert_eq!(is_protein(x), true);
        let y: u32 = 8;
        assert_eq!(is_protein(y), true);
        let mut z: u32 = 4;
        z += 64;
        assert_eq!(is_protein(z), false);
    }
    #[test]
    fn test_is_north() {
        let mut x: u32 = 7;
        x += 64;
        assert_eq!(is_north(x), true);
        let y: u32 = 8;
        assert_eq!(is_north(y), true);
        let mut z: u32 = 4;
        z += 128;
        assert_eq!(is_north(z), false);
    }
    #[test]
    fn test_is_south() {
        let mut x: u32 = 7;
        x += 64;
        assert_eq!(is_south(x), false);
        let mut y: u32 = 8;
        y += 128;
        assert_eq!(is_south(y), true);
        let mut z: u32 = 4;
        z += 128 + 256;
        assert_eq!(is_south(z), false);
    }
    #[test]
    fn test_is_west() {
        let mut x: u32 = 7;
        x += 64;
        assert_eq!(is_west(x), false);
        let mut y: u32 = 8;
        y += 128;
        assert_eq!(is_west(y), false);
        let mut z: u32 = 4;
        z += 256;
        assert_eq!(is_west(z), true);
        let mut w: u32 = 4;
        w += 256 + 128;
        assert_eq!(is_west(w), false);
    }
    #[test]
    fn test_is_east() {
        let mut x: u32 = 7;
        x += 64;
        assert_eq!(is_east(x), false);
        let mut y: u32 = 8;
        y += 128;
        assert_eq!(is_east(y), false);
        let mut z: u32 = 4;
        z += 256;
        assert_eq!(is_east(z), false);
        let mut w: u32 = 4;
        w += 256 + 128;
        assert_eq!(is_east(w), true);
    }
    #[test]
    fn test_is_mouth() {
        let mut x: u32 = 5;
        x += 64;
        assert_eq!(is_mouth(x), true);
        let mut y: u32 = 5;
        y += 128;
        assert_eq!(is_mouth(y), true);
        let mut z: u32 = 8;
        z += 128;
        assert_eq!(is_mouth(z), false);
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
use arena::{print_map, Arena};
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
pub const NORTH: u32 = 0;
pub const SOUTH: u32 = 128;
pub const WEST: u32 = 256;
pub const EAST: u32 = 256 + 128;
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
            let mut _organ_id = parse_input!(inputs[4], u32);
            let organ_dir = inputs[5].trim().to_string();
            if organ_dir == "S" {
                new_elem += 128;
            } else if organ_dir == "W" {
                new_elem += 256;
            } else if organ_dir == "E" {
                new_elem += 256 + 128;
            }
            let _organ_parent_id = parse_input!(inputs[6], u32);
            let mut _organ_root_id = parse_input!(inputs[7], u32);
            if _organ_root_id > 0 {
                _organ_root_id = _organ_root_id << 16;
                new_elem += _organ_root_id;
            }
            let index = cols * y + x;
            arena.map[index as usize] = new_elem;
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
        for id in 0..required_actions_count as u32 {
            let mut output = String::new();
            let (x_new, y_new, order, direction) = arena.next_move(id * 2 + 1);
            if true {
                output.push_str("GROW ");
                output.push_str(&(id * 2 + 1).to_string());
                output.push_str(" ");
                output.push_str(&x_new.to_string());
                output.push_str(" ");
                output.push_str(&y_new.to_string());
                output.push_str(&order);
                output.push_str(&direction);
                println!("{}", output);
            } else {
                println!("WAIT");
            }
        }
    }
}

