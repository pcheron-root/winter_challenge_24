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
        let nb_a = self.a;
        let nb_harv = std::cmp::min(self.c, self.d);
        let nb_tentacle = std::cmp::min(self.b, self.c);
        let nb_sporer = std::cmp::min(self.b, self.d);

        if nb_a >= nb_harv && nb_a >= nb_tentacle && nb_a >= nb_sporer {
            return (" BASIC".to_string(), "".to_string());
        }
        if nb_sporer >= nb_tentacle && nb_sporer >= nb_harv {
            return (" SPORER".to_string(), " E".to_string());
        }
        if nb_harv >= nb_tentacle {
            return (" HARVESTER".to_string(), " E".to_string());
        }
        if self.b > 0 && self.c > 0 {
            return (" TENTACLE".to_string(), " W".to_string());
        }
        ("WAIT".to_string(), "".to_string())
    }
}
