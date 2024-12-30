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

    pub fn find_right_cel(&self) -> (String, String) {
        if self.a > 0 {
            return (" BASIC".to_string(), "".to_string());
        } else if self.b > 0 && self.c > 0 {
            return (" TENTACLE".to_string(), " W".to_string());
        } else if self.b > 0 && self.d > 0 {
            return (" SPORER".to_string(), " E".to_string());
        } else if self.c > 0 && self.d > 0 {
            return (" HARVESTER".to_string(), " E".to_string());
        }
        (" WAIT".to_string(), "".to_string())
    }
}
