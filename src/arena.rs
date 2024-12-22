// use winter::ENEMY;
// use winter::MINE;

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
                        if map[y * self.nb_col + x + 1] > i + 1
                            && map[y * self.nb_col + x + 1] != 64
                        {
                            map[y * self.nb_col + x + 1] = i + 1;
                            if i == 1 && is_oppo(self.map[y * self.nb_col + x + 1]) {
                                return (true, x, y, " TENTACLE".to_string(), " E".to_string());
                            }
                        }
                        if map[y * self.nb_col + x - 1] > i + 1
                            && map[y * self.nb_col + x - 1] != 64
                        {
                            map[y * self.nb_col + x - 1] = i + 1;
                            if i == 1 && is_oppo(self.map[y * self.nb_col + x - 1]) {
                                return (true, x, y, " TENTACLE".to_string(), " W".to_string());
                            }
                        }
                        if map[(y + 1) * self.nb_col + x] > i + 1
                            && map[(y + 1) * self.nb_col + x] != 64
                        {
                            map[(y + 1) * self.nb_col + x] = i + 1;
                            if i == 1 && is_oppo(self.map[(y + 1) * self.nb_col + x]) {
                                return (true, x, y, " TENTACLE".to_string(), " S".to_string());
                            }
                        }
                        if map[(y - 1) * self.nb_col + x] > i + 1
                            && map[(y - 1) * self.nb_col + x] != 64
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
                        if map[y * self.nb_col + x + 1] > i + 1
                            && map[y * self.nb_col + x + 1] != 64
                        {
                            map[y * self.nb_col + x + 1] = i + 1;
                            if i == 1
                                && is_protein(self.map[y * self.nb_col + x + 1])
                                && !self.is_ate(x + 1, y)
                            {
                                return (x, y, " HARVESTER".to_string(), " E".to_string());
                            }
                        }
                        if map[y * self.nb_col + x - 1] > i + 1
                            && map[y * self.nb_col + x - 1] != 64
                        {
                            map[y * self.nb_col + x - 1] = i + 1;
                            if i == 1
                                && is_protein(self.map[y * self.nb_col + x - 1])
                                && !self.is_ate(x - 1, y)
                            {
                                return (x, y, " HARVESTER".to_string(), " W".to_string());
                            }
                        }
                        if map[(y + 1) * self.nb_col + x] > i + 1
                            && map[(y + 1) * self.nb_col + x] != 64
                        {
                            map[(y + 1) * self.nb_col + x] = i + 1;
                            if i == 1
                                && is_protein(self.map[(y + 1) * self.nb_col + x])
                                && !self.is_ate(x, y + 1)
                            {
                                return (x, y, " HARVESTER".to_string(), " S".to_string());
                            }
                        }
                        if map[(y - 1) * self.nb_col + x] > i + 1
                            && map[(y - 1) * self.nb_col + x] != 64
                        {
                            map[(y - 1) * self.nb_col + x] = i + 1;
                            if i == 1
                                && is_protein(self.map[(y + 1) * self.nb_col + x])
                                && !self.is_ate(x, y + 1)
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
        // j'ai rien trouve
        return (0, 0, " BASIC".to_string(), "".to_string());
    }

    pub fn is_ate(&self, x: usize, y: usize) -> bool {
        if is_mouth(self.map[(y - 1) * self.nb_col + x])
            && is_south(self.map[(y - 1) * self.nb_col + x])
        {
            return true;
        } else if is_mouth(self.map[(y + 1) * self.nb_col + x])
            && is_north(self.map[(y + 1) * self.nb_col + x])
        {
            return true;
        } else if is_mouth(self.map[y * self.nb_col + x - 1])
            && is_east(self.map[y * self.nb_col + x - 1])
        {
            return true;
        } else if is_mouth(self.map[y * self.nb_col + x + 1])
            && is_west(self.map[y * self.nb_col + x + 1])
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
    let mut x: u32 = 3; // basic
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
    let mut x: u32 = 3; // basic
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
    let mut x: u32 = 1; // basic
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
    let mut x: u32 = 7; // basic
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
    let mut x: u32 = 7; // basic
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
    let mut x: u32 = 7; // basic
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
    let mut x: u32 = 7; // basic
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
    let mut x: u32 = 7; // basic
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
