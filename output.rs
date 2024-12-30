pub mod arena {
    use super::Player;
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
        pub fn find_my_id(&self, x: usize, y: usize, origin: u32) -> u32 {
            if x + 1 < self.nb_col && is_mine(self.map[y * self.nb_col + x + 1]) &&
                self.map[y * self.nb_col + x + 1] << 11 >> 21 == origin
            {
                return self.map[y * self.nb_col + x + 1] >> 21;
            } else if x > 0 && is_mine(self.map[y * self.nb_col + x - 1]) &&
                       self.map[y * self.nb_col + x - 1] << 11 >> 21 == origin
            {
                return self.map[y * self.nb_col + x - 1] >> 21;
            } else if y + 1 < self.nb_lin && is_mine(self.map[(y + 1) * self.nb_col + x]) &&
                       self.map[(y + 1) * self.nb_col + x] << 11 >> 21 == origin
            {
                return self.map[(y + 1) * self.nb_col + x] >> 21;
            } else if y > 0 && is_mine(self.map[(y - 1) * self.nb_col + x]) &&
                       self.map[(y - 1) * self.nb_col + x] << 11 >> 21 == origin
            {
                return self.map[(y - 1) * self.nb_col + x] >> 21;
            }
            0
        }
        pub fn is_enemy_near(&self, id: u32) -> (bool, usize, usize, String, String, u32) {
            let mut map = vec![5; self.nb_col * self.nb_lin];
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
            for i in 0..3 {
                for y in 0..self.nb_lin {
                    for x in 0..self.nb_col {
                        if map[y * self.nb_col + x] == i {
                            if x + 1 < self.nb_col && map[y * self.nb_col + x + 1] > i + 1 &&
                                map[y * self.nb_col + x + 1] != 64
                            {
                                map[y * self.nb_col + x + 1] = i + 1;
                            }
                            if x >= 1 && map[y * self.nb_col + x - 1] > i + 1 &&
                                map[y * self.nb_col + x - 1] != 64
                            {
                                map[y * self.nb_col + x - 1] = i + 1;
                            }
                            if y + 1 < self.nb_lin && map[(y + 1) * self.nb_col + x] > i + 1 &&
                                map[(y + 1) * self.nb_col + x] != 64
                            {
                                map[(y + 1) * self.nb_col + x] = i + 1;
                            }
                            if y >= 1 && map[(y - 1) * self.nb_col + x] > i + 1 &&
                                map[(y - 1) * self.nb_col + x] != 64
                            {
                                map[(y - 1) * self.nb_col + x] = i + 1;
                            }
                        }
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 1 {
                        if is_oppo(self.map[y * self.nb_col + x]) {
                            let (is_already_attack, _dir) = self.is_targetate_by_my_tentacle(x, y);
                            if is_already_attack {
                                if x + 1 < self.nb_col &&
                                    is_oppo(self.map[y * self.nb_col + x + 1]) &&
                                    !self.is_forbidden_move(x, y)
                                {
                                    eprintln!("close combat");
                                    return (
                                        true,
                                        x,
                                        y,
                                        " TENTACLE".to_string(),
                                        " E".to_string(),
                                        self.find_my_id(x, y, id),
                                    );
                                } else if x > 0 && is_oppo(self.map[y * self.nb_col + x - 1]) &&
                                           !self.is_forbidden_move(x, y)
                                {
                                    eprintln!("close combat");
                                    return (
                                        true,
                                        x,
                                        y,
                                        " TENTACLE".to_string(),
                                        " W".to_string(),
                                        self.find_my_id(x, y, id),
                                    );
                                } else if y > 0 && is_oppo(self.map[(y - 1) * self.nb_col + x]) &&
                                           !self.is_forbidden_move(x, y)
                                {
                                    eprintln!("close combat");
                                    return (
                                        true,
                                        x,
                                        y,
                                        " TENTACLE".to_string(),
                                        " N".to_string(),
                                        self.find_my_id(x, y, id),
                                    );
                                } else if y + 1 < self.nb_lin &&
                                           is_oppo(self.map[(y + 1) * self.nb_col + x]) &&
                                           !self.is_forbidden_move(x, y)
                                {
                                    eprintln!("close combat");
                                    return (
                                        true,
                                        x,
                                        y,
                                        " TENTACLE".to_string(),
                                        " S".to_string(),
                                        self.find_my_id(x, y, id),
                                    );
                                }
                            }
                            map[y * self.nb_col + x] = 2;
                        }
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 2 {
                        if is_oppo(self.map[y * self.nb_col + x]) {
                            if x + 1 < self.nb_col {
                                if map[y * self.nb_col + x + 1] == 1 &&
                                    !self.is_forbidden_move(x + 1, y)
                                {
                                    eprintln!("combat 2");
                                    return (
                                        true,
                                        x + 1,
                                        y,
                                        " TENTACLE".to_string(),
                                        " W".to_string(),
                                        self.find_my_id(x + 1, y, id),
                                    );
                                }
                            }
                            if x > 0 {
                                if map[y * self.nb_col + x - 1] == 1 &&
                                    !self.is_forbidden_move(x - 1, y)
                                {
                                    eprintln!("combat 2");
                                    return (
                                        true,
                                        x - 1,
                                        y,
                                        " TENTACLE".to_string(),
                                        " E".to_string(),
                                        self.find_my_id(x - 1, y, id),
                                    );
                                }
                            }
                            if y + 1 < self.nb_lin && !self.is_forbidden_move(x, y + 1) {
                                if map[(y + 1) * self.nb_col + x] == 1 {
                                    eprintln!("combat 2");
                                    return (
                                        true,
                                        x,
                                        y + 1,
                                        " TENTACLE".to_string(),
                                        " N".to_string(),
                                        self.find_my_id(x, y + 1, id),
                                    );
                                }
                            }
                            if y > 0 {
                                if map[(y - 1) * self.nb_col + x] == 1 &&
                                    !self.is_forbidden_move(x, y - 1)
                                {
                                    eprintln!("combat 2");
                                    return (
                                        true,
                                        x,
                                        y - 1,
                                        " TENTACLE".to_string(),
                                        " S".to_string(),
                                        self.find_my_id(x, y - 1, id),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 2 && self.is_enemy_next_to(x, y) {
                        if x + 1 < self.nb_col {
                            if map[y * self.nb_col + x + 1] == 1 &&
                                !self.is_forbidden_move(x + 1, y)
                            {
                                eprintln!("combat 3");
                                return (
                                    true,
                                    x + 1,
                                    y,
                                    " TENTACLE".to_string(),
                                    " W".to_string(),
                                    self.find_my_id(x + 1, y, id),
                                );
                            }
                        }
                        if x > 0 {
                            if map[y * self.nb_col + x - 1] == 1 &&
                                !self.is_forbidden_move(x - 1, y)
                            {
                                eprintln!("combat 3");
                                return (
                                    true,
                                    x - 1,
                                    y,
                                    " TENTACLE".to_string(),
                                    " E".to_string(),
                                    self.find_my_id(x - 1, y, id),
                                );
                            }
                        }
                        if y + 1 < self.nb_lin && !self.is_forbidden_move(x, y + 1) {
                            if map[(y + 1) * self.nb_col + x] == 1 {
                                eprintln!("combat 3");
                                return (
                                    true,
                                    x,
                                    y + 1,
                                    " TENTACLE".to_string(),
                                    " N".to_string(),
                                    self.find_my_id(x, y + 1, id),
                                );
                            }
                        }
                        if y > 0 {
                            if map[(y - 1) * self.nb_col + x] == 1 &&
                                !self.is_forbidden_move(x, y - 1)
                            {
                                eprintln!("combat 3");
                                return (
                                    true,
                                    x,
                                    y - 1,
                                    " TENTACLE".to_string(),
                                    " S".to_string(),
                                    self.find_my_id(x, y - 1, id),
                                );
                            }
                        }
                    }
                }
            }
            eprint!("I don't find tentacle to summon");
            print_map(map.clone(), self.nb_col, self.nb_lin);
            return (false, 0, 0, "".to_string(), "".to_string(), 0);
        }
        pub fn find_right_id(&self, mut num_id: u32) -> u32 {
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if is_root(self.map[y * self.nb_col + x]) &&
                        is_mine(self.map[y * self.nb_col + x])
                    {
                        if num_id > 0 {
                            num_id -= 1;
                        } else {
                            return self.map[y * self.nb_col + x] >> 21;
                        }
                    }
                }
            }
            0
        }
        pub fn next_move(
            &self,
            num_id: u32,
            guapo: &Player,
            oppo: &Player,
        ) -> (u32, usize, usize, String, String) {
            let id = self.find_right_id(num_id);
            if guapo.b > 0 && guapo.c > 0 {
                let (is_near, x, y, order, direction, new_id) = self.is_enemy_near(id);
                if is_near {
                    return (new_id, x, y, order, direction);
                }
            }
            return self.looking_for_prot(id, guapo, oppo);
        }
        pub fn find_stupid_move(
            &self,
            id: u32,
            guapo: &Player,
        ) -> (u32, usize, usize, String, String) {
            let mut map = vec![4; self.nb_col * self.nb_lin];
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if is_wall(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 64;
                    } else if is_from_organ(self.map[y * self.nb_col + x], id) {
                        map[y * self.nb_col + x] = 0;
                    } else if is_mine(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 64;
                    } else if is_oppo(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 32;
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 0 {
                        if x + 1 < self.nb_col && map[y * self.nb_col + x + 1] == 4 {
                            map[y * self.nb_col + x + 1] = 1 +
                                is_protein(self.map[y * self.nb_col + x + 1]) as u32;
                        }
                        if x > 0 && map[y * self.nb_col + x - 1] == 4 {
                            map[y * self.nb_col + x - 1] = 1 +
                                is_protein(self.map[y * self.nb_col + x - 1]) as u32;
                        }
                        if y + 1 < self.nb_lin && map[(y + 1) * self.nb_col + x] == 4 {
                            map[(y + 1) * self.nb_col + x] = 1 +
                                is_protein(self.map[(y + 1) * self.nb_col + x]) as u32;
                        }
                        if y > 0 && map[(y - 1) * self.nb_col + x] == 4 {
                            map[(y - 1) * self.nb_col + x] = 1 +
                                is_protein(self.map[(y - 1) * self.nb_col + x]) as u32;
                        }
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 1 {
                        if guapo.a > 0 {
                            return (id, x, y, " BASIC".to_string(), "".to_string());
                        } else if guapo.b > 0 && guapo.c > 0 {
                            return (id, x, y, " TENTACLE".to_string(), " W".to_string());
                        } else if guapo.b > 0 && guapo.d > 0 {
                            return (id, x, y, " SPORER".to_string(), " E".to_string());
                        } else if guapo.c > 0 && guapo.d > 0 {
                            return (id, x, y, " HARVESTER".to_string(), " E".to_string());
                        }
                    }
                }
            }
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 2 {
                        if guapo.a > 0 {
                            return (id, x, y, " BASIC".to_string(), "".to_string());
                        } else if guapo.b > 0 && guapo.c > 0 {
                            return (id, x, y, " TENTACLE".to_string(), " W".to_string());
                        } else if guapo.b > 0 && guapo.d > 0 {
                            return (id, x, y, " SPORER".to_string(), " E".to_string());
                        } else if guapo.c > 0 && guapo.d > 0 {
                            return (id, x, y, " HARVESTER".to_string(), " E".to_string());
                        }
                    }
                }
            }
            return (0, 0, 0, "WAIT".to_string(), "".to_string());
        }
        pub fn looking_for_prot(
            &self,
            id: u32,
            guapo: &Player,
            _oppo: &Player,
        ) -> (u32, usize, usize, String, String) {
            let mut map = vec![4; self.nb_col * self.nb_lin];
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if is_wall(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 64;
                    }
                    if is_from_organ(self.map[y * self.nb_col + x], id) {
                        map[y * self.nb_col + x] = 0;
                    } else if is_mine(self.map[y * self.nb_col + x]) {
                        map[y * self.nb_col + x] = 32;
                    }
                }
            }
            for i in 0..7 {
                for y in 0..self.nb_lin {
                    for x in 0..self.nb_col {
                        if map[y * self.nb_col + x] == i {
                            if guapo.c > 0 && guapo.d > 0 {
                                if x + 1 < self.nb_col && map[y * self.nb_col + x + 1] > i + 1 &&
                                    map[y * self.nb_col + x + 1] != 64
                                {
                                    map[y * self.nb_col + x + 1] = i + 1;
                                    if i == 1 && is_protein(self.map[y * self.nb_col + x + 1]) &&
                                        !self.is_ate(x + 1, y) &&
                                        !self.is_ate(x, y) &&
                                        !self.is_forbidden_move(x, y)
                                    {
                                        return (
                                            self.find_my_id(x, y, id),
                                            x,
                                            y,
                                            " HARVESTER".to_string(),
                                            " E".to_string(),
                                        );
                                    }
                                }
                                if x > 0 && map[y * self.nb_col + x - 1] > i + 1 &&
                                    map[y * self.nb_col + x - 1] != 64
                                {
                                    map[y * self.nb_col + x - 1] = i + 1;
                                    if i == 1 && is_protein(self.map[y * self.nb_col + x - 1]) &&
                                        !self.is_ate(x - 1, y) &&
                                        !self.is_ate(x, y) &&
                                        !self.is_forbidden_move(x, y)
                                    {
                                        return (
                                            self.find_my_id(x, y, id),
                                            x,
                                            y,
                                            " HARVESTER".to_string(),
                                            " W".to_string(),
                                        );
                                    }
                                }
                                if y + 1 < self.nb_lin && map[(y + 1) * self.nb_col + x] > i + 1 &&
                                    map[(y + 1) * self.nb_col + x] != 64
                                {
                                    map[(y + 1) * self.nb_col + x] = i + 1;
                                    if i == 1 && is_protein(self.map[(y + 1) * self.nb_col + x]) &&
                                        !self.is_ate(x, y + 1) &&
                                        !self.is_ate(x, y) &&
                                        !self.is_forbidden_move(x, y)
                                    {
                                        return (
                                            self.find_my_id(x, y, id),
                                            x,
                                            y,
                                            " HARVESTER".to_string(),
                                            " S".to_string(),
                                        );
                                    }
                                }
                                if y > 0 && map[(y - 1) * self.nb_col + x] > i + 1 &&
                                    map[(y - 1) * self.nb_col + x] != 64
                                {
                                    map[(y - 1) * self.nb_col + x] = i + 1;
                                    if i == 1 && is_protein(self.map[(y - 1) * self.nb_col + x]) &&
                                        !self.is_ate(x, y - 1) &&
                                        !self.is_ate(x, y) &&
                                        !self.is_forbidden_move(x, y)
                                    {
                                        return (
                                            self.find_my_id(x, y, id),
                                            x,
                                            y,
                                            " HARVESTER".to_string(),
                                            " N".to_string(),
                                        );
                                    }
                                }
                            }
                            if is_protein(self.map[y * self.nb_col + x]) && !self.is_ate(x, y) &&
                                !self.is_forbidden_move(x, y)
                            {
                                if i == 1 {
                                    map[y * self.nb_col + x] = 2;
                                } else {
                                    if guapo.a > 0 {
                                        return (id, x, y, " BASIC".to_string(), "".to_string());
                                    } else if guapo.b > 0 && guapo.c > 0 {
                                        return (
                                            id,
                                            x,
                                            y,
                                            " TENTACLE".to_string(),
                                            " W".to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return self.find_stupid_move(id, guapo);
        }
        pub fn is_enemy_next_to(&self, x: usize, y: usize) -> bool {
            if y > 0 && is_oppo(self.map[(y - 1) * self.nb_col + x]) {
                return true;
            } else if y + 1 < self.nb_lin && is_oppo(self.map[(y + 1) * self.nb_col + x]) {
                return true;
            } else if x > 0 && is_oppo(self.map[y * self.nb_col + x - 1]) {
                return true;
            } else if x + 1 < self.nb_col && is_oppo(self.map[y * self.nb_col + x + 1]) {
                return true;
            }
            false
        }
        pub fn is_organism_next_to(&self, x: usize, y: usize) -> bool {
            let mut map = vec![6; self.nb_col * self.nb_lin];
            for i in 0..self.nb_lin {
                for j in 0..self.nb_col {
                    if is_wall(self.map[i * self.nb_col + j]) {
                        map[i * self.nb_col + j] = 8;
                    }
                    if is_mine(self.map[i * self.nb_col + j]) {
                        map[i * self.nb_col + j] = 64;
                    }
                    if is_oppo(self.map[i * self.nb_col + j]) {
                        map[i * self.nb_col + j] = 32;
                    }
                }
            }
            map[y * self.nb_col + x] = 0;
            for i in 0..5 {
                for j in 0..self.nb_lin {
                    for k in 0..self.nb_col {
                        if map[j * self.nb_col + k] == i {
                            if k + 1 < self.nb_col {
                                if map[j * self.nb_col + k + 1] > 8 {
                                    return true;
                                } else if map[j * self.nb_col + k + 1] < 8 {
                                    map[j * self.nb_col + k + 1] = i + 1
                                }
                            }
                            if k > 0 {
                                if map[j * self.nb_col + k - 1] > 8 {
                                    return true;
                                } else if map[j * self.nb_col + k - 1] < 8 {
                                    map[j * self.nb_col + k - 1] = i + 1
                                }
                            }
                            if j + 1 < self.nb_lin {
                                if map[(j + 1) * self.nb_col + k] > 8 {
                                    return true;
                                } else if map[(j + 1) * self.nb_col + k] < 8 {
                                    map[(j + 1) * self.nb_col + k] = i + 1
                                }
                            }
                            if j > 0 {
                                if map[(j - 1) * self.nb_col + k] > 8 {
                                    return true;
                                } else if map[(j - 1) * self.nb_col + k] < 8 {
                                    map[(j - 1) * self.nb_col + k] = i + 1
                                }
                            }
                        }
                    }
                }
            }
            false
        }
        pub fn is_ate(&self, x: usize, y: usize) -> bool {
            if y > 0 && is_mouth(self.map[(y - 1) * self.nb_col + x]) &&
                is_south(self.map[(y - 1) * self.nb_col + x])
            {
                return true;
            } else if y + 1 < self.nb_lin && is_mouth(self.map[(y + 1) * self.nb_col + x]) &&
                       is_north(self.map[(y + 1) * self.nb_col + x])
            {
                return true;
            } else if x > 0 && is_mouth(self.map[y * self.nb_col + x - 1]) &&
                       is_east(self.map[y * self.nb_col + x - 1])
            {
                return true;
            } else if x + 1 < self.nb_col && is_mouth(self.map[y * self.nb_col + x + 1]) &&
                       is_west(self.map[y * self.nb_col + x + 1])
            {
                return true;
            }
            false
        }
        pub fn is_tentacled(&self, x: usize, y: usize) -> bool {
            if y > 0 && is_tentacle(self.map[(y - 1) * self.nb_col + x]) &&
                is_oppo(self.map[(y - 1) * self.nb_col + x]) &&
                is_south(self.map[(y - 1) * self.nb_col + x])
            {
                return true;
            } else if y + 1 < self.nb_lin && is_tentacle(self.map[(y + 1) * self.nb_col + x]) &&
                       is_oppo(self.map[(y + 1) * self.nb_col + x]) &&
                       is_north(self.map[(y + 1) * self.nb_col + x])
            {
                return true;
            } else if x > 0 && is_tentacle(self.map[y * self.nb_col + x - 1]) &&
                       is_oppo(self.map[y * self.nb_col + x - 1]) &&
                       is_east(self.map[y * self.nb_col + x - 1])
            {
                return true;
            } else if x + 1 < self.nb_col && is_tentacle(self.map[y * self.nb_col + x + 1]) &&
                       is_oppo(self.map[y * self.nb_col + x + 1]) &&
                       is_west(self.map[y * self.nb_col + x + 1])
            {
                return true;
            }
            false
        }
        pub fn is_forbidden_move(&self, x: usize, y: usize) -> bool {
            if is_wall(self.map[y * self.nb_col + x]) {
                return true;
            }
            if self.is_tentacled(x, y) {
                return true;
            }
            false
        }
        pub fn is_targetate_by_my_tentacle(&self, x: usize, y: usize) -> (bool, u32) {
            if x + 1 < self.nb_col {
                let elem = self.map[y * self.nb_col + x + 1];
                if is_tentacle(elem) && is_mine(elem) && is_west(elem) {
                    return (true, 256);
                }
            }
            if x > 0 {
                let elem = self.map[y * self.nb_col + x - 1];
                if is_tentacle(elem) && is_mine(elem) && is_east(elem) {
                    return (true, 256 + 128);
                }
            }
            if y + 1 < self.nb_lin {
                let elem = self.map[(y + 1) * self.nb_col + x + 1];
                if is_tentacle(elem) && is_mine(elem) && is_north(elem) {
                    return (true, 0);
                }
            }
            if y > 0 {
                let elem = self.map[(y - 1) * self.nb_col + x];
                if is_tentacle(elem) && is_mine(elem) && is_south(elem) {
                    return (true, 128);
                }
            }
            (false, 0)
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
        id == (x << 11 >> 21)
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
            map[j] = map[j] << 11 >> 21;
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
    pub fn is_tentacle(x: u32) -> bool {
        if x << 27 >> 27 == 4 {
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
    pub fn is_free(x: u32) -> bool {
        if (x & 32) == 32 || (x & 64) == 64 {
            return false;
        }
        true
    }
    pub fn is_crossable(x: u32) -> bool {
        if x == 0 || is_protein(x) {
            return true;
        }
        false
    }
    pub fn is_sporer(x: u32) -> bool {
        if x & 6 == 6 {
            return true;
        }
        false
    }
    pub fn is_root(x: u32) -> bool {
        if x << 27 >> 27 == 2 {
            return true;
        }
        false
    }
    #[cfg(test)]
    mod tests {
        use super::*;
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
            let w: u32 = 6;
            assert_eq!(is_protein(w), false);
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
        #[test]
        fn test_is_free() {
            let mut x: u32 = 5;
            x += 64;
            assert_eq!(is_free(x), false);
            let mut y: u32 = 5;
            y += 128;
            assert_eq!(is_free(y), true);
            let mut z: u32 = 8;
            z += 32;
            assert_eq!(is_free(z), false);
        }
        #[test]
        fn test_is_crossable() {
            let mut x: u32 = 5;
            x += 64;
            assert_eq!(is_crossable(x), false);
            let mut y: u32 = 6;
            y += 128;
            assert_eq!(is_crossable(y), false);
            let z: u32 = 8;
            assert_eq!(is_crossable(z), true);
            let w: u32 = 0;
            assert_eq!(is_crossable(w), true);
            let v: u32 = 8;
            assert_eq!(is_crossable(v), true);
        }
        #[test]
        fn test_is_sporer() {
            let mut x: u32 = 5;
            x += 64;
            assert_eq!(is_sporer(x), false);
            let mut y: u32 = 6;
            y += 128;
            assert_eq!(is_sporer(y), true);
            let z: u32 = 6;
            assert_eq!(is_sporer(z), true);
            let w: u32 = 0;
            assert_eq!(is_sporer(w), false);
            let v: u32 = 8;
            assert_eq!(is_sporer(v), false);
        }
        #[test]
        fn test_is_root() {
            let mut x: u32 = 5;
            x += 64;
            assert_eq!(is_root(x), false);
            let mut y: u32 = 2;
            y += 128;
            assert_eq!(is_root(y), true);
            let z: u32 = 2;
            assert_eq!(is_root(z), true);
            let w: u32 = 0;
            assert_eq!(is_root(w), false);
            let v: u32 = 6;
            assert_eq!(is_root(v), false);
        }
        #[test]
        fn test_is_tentacled() {
            let mut x: u32 = 5;
            x += 64;
            assert_eq!(is_tentacle(x), false);
            let mut y: u32 = 4;
            y += 128;
            assert_eq!(is_tentacle(y), true);
            let z: u32 = 4;
            assert_eq!(is_tentacle(z), true);
            let w: u32 = 0;
            assert_eq!(is_tentacle(w), false);
            let v: u32 = 6;
            assert_eq!(is_tentacle(v), false);
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

