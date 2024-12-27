// pb a gerer
// j'utilise les mauvais id root

pub mod arena;
pub mod player;

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

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn update_money(guapo: &mut Player, order: String) {
    if order == " ROOT" {
        guapo.a -= 1;
        guapo.b -= 1;
        guapo.c -= 1;
        guapo.d -= 1;
    } else if order == " TENTACLE" {
        guapo.b -= 1;
        guapo.c -= 1;
    } else if order == " BASIC" {
        guapo.a -= 1;
    } else if order == " HARVESTER" {
        guapo.c -= 1;
        guapo.d -= 1;
    } else if order == " SPORER" {
        guapo.b -= 1;
        guapo.d -= 1;
    }
}
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

            // 1 if your organ, 0 if enemy organ, -1 if neither -> 3
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

            // id of this entity if it's an organ, 0 otherwise ->
            let mut organ_id = parse_input!(inputs[4], u32);
            organ_id = organ_id << 21;
            new_elem += organ_id;

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
                _organ_root_id = _organ_root_id << 10;
                new_elem += _organ_root_id;
            }

            let index = cols * y + x;
            arena.map[index as usize] = new_elem;
        }

        // my protein stock
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_a = parse_input!(inputs[0], usize);
        let my_b = parse_input!(inputs[1], usize);
        let my_c = parse_input!(inputs[2], usize);
        let my_d = parse_input!(inputs[3], usize);
        guapo.update_values(my_a, my_b, my_c, my_d);

        // opponent protein stock
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opp_a = parse_input!(inputs[0], usize);
        let opp_b = parse_input!(inputs[1], usize);
        let opp_c = parse_input!(inputs[2], usize);
        let opp_d = parse_input!(inputs[3], usize);
        opponent.update_values(opp_a, opp_b, opp_c, opp_d);

        // number of organisms
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let required_actions_count = parse_input!(input_line, i32); // your number of organisms, output an action for each one in any order

        for num_id in 0..required_actions_count as u32 {
            let mut output = String::new();
            let (id, x_new, y_new, order, direction) = arena.next_move(num_id, &guapo, &opponent);
            if order == "SPORE " {
                output.push_str("SPORE ");
                output.push_str(&direction);
                output.push_str(" ");
                output.push_str(&x_new.to_string());
                output.push_str(" ");
                output.push_str(&y_new.to_string());
                // guapo.b -= 1;
                // guapo.d -= 1;
            } else {
                output.push_str("GROW ");
                output.push_str(&(id).to_string());
                output.push_str(" ");
                output.push_str(&x_new.to_string());
                output.push_str(" ");
                output.push_str(&y_new.to_string());
                output.push_str(&order);
                output.push_str(&direction);
            }
            if order == "WAIT" {
                println!("WAIT");
            } else {
                update_money(&mut guapo, order);
                println!("{}", output);
            }
        }
    }
}
