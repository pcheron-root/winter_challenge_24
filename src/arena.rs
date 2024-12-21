// use winter::ENEMY;
// use winter::MINE;

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

    // x -> col
    // y -> ligne
    // pub fn is_my_cel_next_to(&self, x: usize, y: usize) -> (bool, usize, usize) {
    //     if x > 0 && self.is_mine(x - 1, y) {
    //         return (true, x - 1, y);
    //     } else if self.is_mine(x + 1, y) {
    //         return (true, x + 1, y);
    //     } else if self.is_mine(x, y + 1) {
    //         return (true, x, y + 1);
    //     } else if y > 0 && self.is_mine(x, y - 1) {
    //         return (true, x, y - 1);
    //     }
    //     (false, 0, 0)
    // }

    // pub fn find_where_grow(&self) -> (bool, usize, usize, u32) {
    //     for y in 0..self.dim_y {
    //         for x in 0..self.dim_x {
    //             if self.is_mine(x, y) == false {
    //                 let (is_my_cel, next_x, next_y) = self.is_my_cel_next_to(x, y);
    //                 if is_my_cel {
    //                     return (true, x, y, 0);
    //                 }
    //             }
    //         }
    //     }
    //     (false, 0, 0, 0)
    // }

    // pub fn can_be_won_over(&self, x: usize, y: usize) -> bool {
    //     let mut elem = self.map[y * self.dim_x + x];
    //     elem = elem << 27;
    //     elem = elem >> 27;
    //     if elem == 0 || (7 <= elem && elem <= 11) {
    //         return true;
    //     }
    //     false
    // }

    // pub fn is_an_organ(&self, x: usize, y: usize) -> bool {
    //     let mut elem = self.map[y * self.dim_x + x];
    //     elem = elem << 27;
    //     elem = elem >> 27;
    //     if 2 <= elem && elem <= 6 {
    //         return true;
    //     }
    //     false
    // }

    // pub fn find_strategy(&self, id) -> ( usize, usize) {

    // }

    pub fn looking_for_prot(&self) {
        let min: usize = 100;
        let min_x: usize;
        let min_y: usize;

        let mut map = vec![4; self.dim_x * self.dim_y];
        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                if is_wall(self.map[y * self.dim_x + x]) {
                    map[y * self.dim_x + x] = 64;
                }
                if is_mine(self.map[y * self.dim_x + x]) {
                    map[y * self.dim_x + x] = 0;
                }
            }
        }
        for i in 0..3 {
            for y in 0..self.dim_y {
                for x in 0..self.dim_x {
                    if map[y * self.dim_x + x] == i {
                        //essayer de reduire les nombres des 4 cotes

                        // si j'ai une prot, go envoyer les cohordonnees
                    }
                }
            }
        }
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
