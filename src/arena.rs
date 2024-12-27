// ! tentacule 3 ont des problemes d'orientation
//   il faut comprendre pourquoi ca merde et le fix

// ! faire une instruction au pif si jamais j'ai rien trouve de mieux a faire
//   attention, mettre un basic sur une prot est moins bien que de mettre un basic ailleur
//   si rien a faire, utiliser la commande wait

// ! ameliorer les sporer
//   il faut detechter les endroits loins et proches de 2 des proteines

// ! fix les bouches

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

    pub fn is_enemy_near(&self, id: u32) -> (bool, usize, usize, String, String) {
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
                    if i == 2 {
                        if is_oppo(self.map[y * self.nb_col + x]) {
                            if x + 1 < self.nb_col {
                                if map[y * self.nb_col + x + 1] == 1
                                    && !self.is_forbidden_move(x + 1, y)
                                {
                                    return (
                                        true,
                                        x + 1,
                                        y,
                                        " TENTACLE".to_string(),
                                        " W".to_string(),
                                    );
                                }
                            }
                            if x > 0 {
                                if map[y * self.nb_col + x - 1] == 1
                                    && !self.is_forbidden_move(x - 1, y)
                                {
                                    return (
                                        true,
                                        x + 1,
                                        y,
                                        " TENTACLE".to_string(),
                                        " E".to_string(),
                                    );
                                }
                            }
                            if y + 1 < self.nb_lin && !self.is_forbidden_move(x, y + 1) {
                                if map[(y + 1) * self.nb_col + x] == 1 {
                                    return (
                                        true,
                                        x,
                                        y + 1,
                                        " TENTACLE".to_string(),
                                        " N".to_string(),
                                    );
                                }
                            }
                            if y > 0 {
                                if map[(y - 1) * self.nb_col + x] == 1
                                    && !self.is_forbidden_move(x, y - 1)
                                {
                                    return (
                                        true,
                                        x,
                                        y - 1,
                                        " TENTACLE".to_string(),
                                        " S".to_string(),
                                    );
                                }
                            }
                            // si un mechant autour du 2, creer un tentacule sur le 1 proche du 2
                        }
                    }
                    if map[y * self.nb_col + x] == i {
                        // si 1 est mechant -> devient 2
                        if i == 1 {
                            if is_oppo(self.map[y * self.nb_col + x]) {
                                map[y * self.nb_col + x] = 2;
                            }
                        }
                        //
                        if x + 1 < self.nb_col
                            && map[y * self.nb_col + x + 1] > i + 1
                            && map[y * self.nb_col + x + 1] != 64
                        {
                            map[y * self.nb_col + x + 1] = i + 1;
                        }
                        if x >= 1
                            && map[y * self.nb_col + x - 1] > i + 1
                            && map[y * self.nb_col + x - 1] != 64
                        {
                            map[y * self.nb_col + x - 1] = i + 1;
                        }
                        if y + 1 < self.nb_lin
                            && map[(y + 1) * self.nb_col + x] > i + 1
                            && map[(y + 1) * self.nb_col + x] != 64
                        {
                            map[(y + 1) * self.nb_col + x] = i + 1;
                        }
                        if y >= 1
                            && map[(y - 1) * self.nb_col + x] > i + 1
                            && map[(y - 1) * self.nb_col + x] != 64
                        {
                            map[(y - 1) * self.nb_col + x] = i + 1;
                        }
                    }
                }
            }
        }
        for y in 0..self.nb_lin {
            for x in 0..self.nb_col {
                if map[y * self.nb_col + x] == 2 && self.is_enemy_next_to(x, y) {
                    if x + 1 < self.nb_col {
                        if map[y * self.nb_col + x + 1] == 1 && !self.is_forbidden_move(x + 1, y) {
                            return (true, x + 1, y, " TENTACLE".to_string(), " W".to_string());
                        }
                    }
                    if x > 0 {
                        if map[y * self.nb_col + x - 1] == 1 && !self.is_forbidden_move(x - 1, y) {
                            return (true, x + 1, y, " TENTACLE".to_string(), " E".to_string());
                        }
                    }
                    if y + 1 < self.nb_lin && !self.is_forbidden_move(x, y + 1) {
                        if map[(y + 1) * self.nb_col + x] == 1 {
                            return (true, x, y + 1, " TENTACLE".to_string(), " N".to_string());
                        }
                    }
                    if y > 0 {
                        if map[(y - 1) * self.nb_col + x] == 1 && !self.is_forbidden_move(x, y - 1)
                        {
                            return (true, x, y - 1, " TENTACLE".to_string(), " S".to_string());
                        }
                    }
                }
            }
        }
        return (false, 0, 0, "".to_string(), "".to_string());
    }

    pub fn is_expandable(&self, id: u32) -> (bool, usize, usize, String, String) {
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
        for y in 0..self.nb_lin {
            for x in 0..self.nb_col {
                if map[y * self.nb_col + x] == 0 {
                    if x + 1 < self.nb_col
                        && map[y * self.nb_col + x + 1] > 1
                        && map[y * self.nb_col + x + 1] < 8
                    {
                        map[y * self.nb_col + x + 1] = 1;
                    }
                    if x > 0 && map[y * self.nb_col + x - 1] > 1 && map[y * self.nb_col + x - 1] < 8
                    {
                        map[y * self.nb_col + x - 1] = 1;
                    }
                    if y + 1 < self.nb_lin
                        && map[(y + 1) * self.nb_col + x] > 1
                        && map[y * self.nb_col + x] < 8
                    {
                        map[(y + 1) * self.nb_col + x] = 1;
                    }
                    if y > 0
                        && map[(y - 1) * self.nb_col + x] > 1
                        && map[(y - 1) * self.nb_col + x] < 8
                    {
                        map[(y - 1) * self.nb_col + x] = 1;
                    }
                }
            }
        }
        for y in 0..self.nb_lin {
            for x in 0..self.nb_col {
                if map[y * self.nb_col + x] == 1 && x + 12 < self.nb_col {
                    for i in 1..13 {
                        if !is_crossable(self.map[y * self.nb_col + x + i]) {
                            break;
                        }
                        if i == 12 && !is_protein(self.map[y * self.nb_col + x + i]) {
                            if !self.is_organism_next_to(x + i, y) && !self.is_forbidden_move(x, y)
                            {
                                return (true, x, y, " SPORER".to_string(), " E".to_string());
                            }
                        }
                    }
                }
            }
        }
        (false, 0, 0, "".to_string(), "".to_string())
    }

    pub fn is_charged(&self, id: u32) -> (bool, usize, usize, String, String) {
        for y in 0..self.nb_lin {
            for x in 0..self.nb_col {
                if is_sporer(self.map[y * self.nb_col + x])
                    && is_from_organ(self.map[y * self.nb_col + x], id)
                {
                    if x + 12 < self.nb_col - 1 && !is_mine(self.map[y * self.nb_col + x + 12]) {
                        let id_sporer = self.map[y * self.nb_col + x] >> 24;
                        return (true, x + 12, y, "SPORE ".to_string(), id_sporer.to_string());
                    }
                }
            }
        }
        (false, 0, 0, "".to_string(), "".to_string())
    }

    pub fn find_right_id(&self, mut num_id: u32) -> u32 {
        for y in 0..self.nb_lin {
            for x in 0..self.nb_col {
                if is_root(self.map[y * self.nb_col + x]) && is_mine(self.map[y * self.nb_col + x])
                {
                    if num_id > 0 {
                        num_id -= 1;
                    } else {
                        return self.map[y * self.nb_col + x] >> 24;
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
            let (is_near, x, y, order, direction) = self.is_enemy_near(id);
            if is_near {
                eprintln!("je me defend");
                return (id, x, y, order, direction);
            }
        }
        if guapo.a > 0 && guapo.b > 0 && guapo.c > 0 && guapo.d > 0 {
            let (is_charged, x, y, order, direction) = self.is_charged(id);
            if is_charged {
                eprintln!("je lance un spore");
                return (id, x, y, order, direction);
            }
        }
        if guapo.b > 0 && guapo.d > 0 {
            let (is_expandable, x, y, order, direction) = self.is_expandable(id);
            if is_expandable {
                eprintln!("je lance un sporer");
                return (id, x, y, order, direction);
            }
        }
        return self.looking_for_prot(id, guapo, oppo);
    }

    pub fn looking_for_prot(
        &self,
        id: u32,
        guapo: &Player,
        oppo: &Player,
    ) -> (u32, usize, usize, String, String) {
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
                        if guapo.c > 0 && guapo.d > 0 {
                            if x + 1 < self.nb_col
                                && map[y * self.nb_col + x + 1] > i + 1
                                && map[y * self.nb_col + x + 1] != 64
                            {
                                map[y * self.nb_col + x + 1] = i + 1;
                                if i == 1
                                    && is_protein(self.map[y * self.nb_col + x + 1])
                                    && !self.is_ate(x + 1, y)
                                    && !self.is_forbidden_move(x, y)
                                {
                                    return (id, x, y, " HARVESTER".to_string(), " E".to_string());
                                }
                            }
                            if x > 0
                                && map[y * self.nb_col + x - 1] > i + 1
                                && map[y * self.nb_col + x - 1] != 64
                            {
                                map[y * self.nb_col + x - 1] = i + 1;
                                if i == 1
                                    && is_protein(self.map[y * self.nb_col + x - 1])
                                    && !self.is_ate(x - 1, y)
                                    && !self.is_forbidden_move(x, y)
                                {
                                    return (id, x, y, " HARVESTER".to_string(), " W".to_string());
                                }
                            }
                            if y + 1 < self.nb_lin
                                && map[(y + 1) * self.nb_col + x] > i + 1
                                && map[(y + 1) * self.nb_col + x] != 64
                            {
                                map[(y + 1) * self.nb_col + x] = i + 1;
                                if i == 1
                                    && is_protein(self.map[(y + 1) * self.nb_col + x])
                                    && !self.is_ate(x, y + 1)
                                    && !self.is_forbidden_move(x, y)
                                {
                                    return (id, x, y, " HARVESTER".to_string(), " S".to_string());
                                }
                            }
                            if y > 0
                                && map[(y - 1) * self.nb_col + x] > i + 1
                                && map[(y - 1) * self.nb_col + x] != 64
                            {
                                map[(y - 1) * self.nb_col + x] = i + 1;
                                if i == 1
                                    && is_protein(self.map[(y + 1) * self.nb_col + x])
                                    && !self.is_ate(x, y + 1)
                                    && !self.is_forbidden_move(x, y)
                                {
                                    return (id, x, y, " HARVESTER".to_string(), " N".to_string());
                                }
                            }
                        }
                        if is_protein(self.map[y * self.nb_col + x])
                            && !self.is_ate(x, y)
                            && !self.is_forbidden_move(x, y)
                        {
                            if i == 1 {
                                map[y * self.nb_col + x] = 2;
                            } else {
                                if guapo.a > 0 {
                                    return (id, x, y, " BASIC".to_string(), "".to_string());
                                } else if guapo.b > 0 && guapo.c > 0 {
                                    return (id, x, y, " TENTACLE".to_string(), " W".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        // ici je veux me developper avec uniquement les ressources que j'ai
        // mais dans n'importe quel sens
        if guapo.a > 0 {
            for y in 0..self.nb_lin {
                for x in 0..self.nb_col {
                    if map[y * self.nb_col + x] == 1
                        && !self.is_ate(x, y)
                        && !self.is_forbidden_move(x, y)
                    {
                        return (id, x, y, " BASIC".to_string(), "".to_string());
                    }
                }
            }
        }
        // && !self.is_forbidden_move(x, y)
        return (id, 0, 0, " BASIC".to_string(), "".to_string());
        // creer nimporte quoi
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
                                // eprintln!("desole un organ est pas loin 1");
                                return true;
                            } else if map[j * self.nb_col + k + 1] < 8 {
                                map[j * self.nb_col + k + 1] = i + 1
                            }
                        }
                        if k > 0 {
                            if map[j * self.nb_col + k - 1] > 8 {
                                // eprintln!("desole un organ est pas loin 2 ");
                                return true;
                            } else if map[j * self.nb_col + k - 1] < 8 {
                                map[j * self.nb_col + k - 1] = i + 1
                            }
                        }
                        if j + 1 < self.nb_lin {
                            if map[(j + 1) * self.nb_col + k] > 8 {
                                // eprintln!("desole un organ est pas loin 3 pour x {} y {}", k, j);
                                return true;
                            } else if map[(j + 1) * self.nb_col + k] < 8 {
                                map[(j + 1) * self.nb_col + k] = i + 1
                            }
                        }
                        if j > 0 {
                            if map[(j - 1) * self.nb_col + k] > 8 {
                                // eprintln!("desole un organ est pas loin 4 ");
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
        if y > 0
            && is_mouth(self.map[(y - 1) * self.nb_col + x])
            && is_south(self.map[(y - 1) * self.nb_col + x])
        {
            return true;
        } else if y + 1 < self.nb_lin
            && is_mouth(self.map[(y + 1) * self.nb_col + x])
            && is_north(self.map[(y + 1) * self.nb_col + x])
        {
            return true;
        } else if x > 0
            && is_mouth(self.map[y * self.nb_col + x - 1])
            && is_east(self.map[y * self.nb_col + x - 1])
        {
            return true;
        } else if x + 1 < self.nb_col
            && is_mouth(self.map[y * self.nb_col + x + 1])
            && is_west(self.map[y * self.nb_col + x + 1])
        {
            return true;
        }
        false
    }

    pub fn is_tentacled(&self, x: usize, y: usize) -> bool {
        if y > 0
            && is_tentacle(self.map[(y - 1) * self.nb_col + x])
            && is_oppo(self.map[(y - 1) * self.nb_col + x])
            && is_south(self.map[(y - 1) * self.nb_col + x])
        {
            return true;
        } else if y + 1 < self.nb_lin
            && is_tentacle(self.map[(y + 1) * self.nb_col + x])
            && is_oppo(self.map[(y + 1) * self.nb_col + x])
            && is_north(self.map[(y + 1) * self.nb_col + x])
        {
            return true;
        } else if x > 0
            && is_tentacle(self.map[y * self.nb_col + x - 1])
            && is_oppo(self.map[y * self.nb_col + x - 1])
            && is_east(self.map[y * self.nb_col + x - 1])
        {
            return true;
        } else if x + 1 < self.nb_col
            && is_tentacle(self.map[y * self.nb_col + x + 1])
            && is_oppo(self.map[y * self.nb_col + x + 1])
            && is_west(self.map[y * self.nb_col + x + 1])
        {
            return true;
        }
        false
    }

    pub fn is_forbidden_move(&self, x: usize, y: usize) -> bool {
        if is_oppo(self.map[y * self.nb_col + x]) {
            return true;
        }
        if self.is_tentacled(x, y) {
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
    id == (x << 8 >> 24)
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

        let w: u32 = 6;
        assert_eq!(is_protein(w), false);
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

    #[test]
    fn test_is_harvester_created_at_the_right_place0() {
        let mut arena = Arena::new(5, 5);
        arena.map[0] = 1;
        arena.map[1] = 1;
        arena.map[2] = 1;
        arena.map[3] = 1;
        arena.map[4] = 1;
        arena.map[5] = 1;
        arena.map[6] = 0;
        arena.map[7] = 0;
        arena.map[8] = 0;
        arena.map[9] = 1;
        arena.map[10] = 1;
        arena.map[11] = 2 + 64;
        arena.map[12] = 0;
        arena.map[13] = 7;
        arena.map[14] = 1;
        arena.map[15] = 1;
        arena.map[16] = 0;
        arena.map[17] = 0;
        arena.map[18] = 0;
        arena.map[19] = 1;
        arena.map[20] = 1;
        arena.map[21] = 1;
        arena.map[22] = 1;
        arena.map[23] = 1;
        arena.map[24] = 1;
        let guapo = Player {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };
        let oppo = Player {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };
        let (_id, x, y, order, direction) = arena.looking_for_prot(0, &guapo, &oppo);
        assert_eq!(x, 2);
        assert_eq!(y, 2);
        assert_eq!(order, " HARVESTER".to_string());
        assert_eq!(direction, " E".to_string());
    }

    #[test]
    fn test_is_harvester_created_at_the_right_place1() {
        let mut arena = Arena::new(5, 5);
        arena.map[0] = 1;
        arena.map[1] = 1;
        arena.map[2] = 1;
        arena.map[3] = 1;
        arena.map[4] = 1;
        arena.map[5] = 1;
        arena.map[6] = 0;
        arena.map[7] = 0;
        arena.map[8] = 0;
        arena.map[9] = 1;
        arena.map[10] = 1;
        arena.map[11] = 2 + 64;
        arena.map[12] = 0;
        arena.map[13] = 0;
        arena.map[14] = 1;
        arena.map[15] = 1;
        arena.map[16] = 0;
        arena.map[17] = 7;
        arena.map[18] = 0;
        arena.map[19] = 1;
        arena.map[20] = 1;
        arena.map[21] = 1;
        arena.map[22] = 1;
        arena.map[23] = 1;
        arena.map[24] = 1;
        let guapo = Player {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };
        let oppo = Player {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };
        let (_id, x, y, order, direction) = arena.looking_for_prot(0, &guapo, &oppo);
        assert_eq!(x, 2);
        assert_eq!(y, 2);
        assert_eq!(order, " HARVESTER".to_string());
        assert_eq!(direction, " S".to_string());
    }

    // D -> 10
    #[test]
    fn test_is_harvester_created_at_the_right_place3() {
        let mut arena = Arena::new(4, 4);
        arena.map[0] = 10;
        arena.map[1] = 1;
        arena.map[2] = 0;
        arena.map[3] = 0;
        arena.map[4] = 1;
        arena.map[5] = 0;
        arena.map[6] = 0;
        arena.map[7] = 10;
        arena.map[8] = 2 + 64;
        arena.map[9] = 3 + 64;
        arena.map[10] = 3 + 64;
        arena.map[11] = 0;
        arena.map[12] = 0;
        arena.map[13] = 0;
        arena.map[14] = 0;
        arena.map[15] = 0;
        let guapo = Player {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };
        let oppo = Player {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };
        let (_id, x, y, order, direction) = arena.looking_for_prot(0, &guapo, &oppo);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
        assert_eq!(order, " HARVESTER".to_string());
        assert_eq!(direction, " E".to_string());
    }
}
