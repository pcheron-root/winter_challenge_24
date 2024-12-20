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

    // x -> col
    // y -> ligne
    pub fn is_my_cel_next_to(&self, x: usize, y: usize) -> bool {
        if x > 0 && self.is_mine(x - 1, y) {
            return true;
        } else if self.is_mine(x + 1, y) {
            return true;
        } else if self.is_mine(x, y + 1) {
            return true;
        } else if y > 0 && self.is_mine(x, y - 1) {
            return true;
        }
        false
    }

    pub fn find_where_grow(&self) -> (bool, usize, usize) {
        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                if self.is_mine(x, y) == false && self.is_my_cel_next_to(x, y) {
                    return (true, x, y);
                }
            }
        }
        (false, 0, 0)
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

    pub fn get_bigger_id(&self) -> u32 {
        let mut max = 0;
        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                if self.is_an_organ(x, y) {
                    let mut elem = self.map[y * self.dim_x + x];
                    elem = elem >> 16;
                    if max < elem {
                        max = elem;
                    }
                }
            }
        }
        max
    }
}
