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
