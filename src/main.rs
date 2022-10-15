use nurimisaki_solver::square::Square;

// Up to 64
struct Board<'sq> {
    init: &'sq [Vec<Square>],
    black: Vec<u64>,
    white: Vec<u64>,
}

#[derive(Debug, Clone, Default)]
struct Stat {
    num_call: u64,
}

impl<'sq> Board<'sq> {
    pub fn new(init: &'sq [Vec<Square>]) -> Self {
        let n = init.len();
        let m = init[0].len();
        assert!(m <= 64);
        assert!(init.iter().all(|v| v.len() == m));
        let mut white = vec![0; n];
        for i in 0..n {
            for j in 0..m {
                if init[i][j] != Square::Blank {
                    white[i] |= 1 << j;
                }
            }
        }
        Self {
            init,
            black: vec![0; n],
            white,
        }
    }
    pub fn finished(&self) -> bool {
        if self.contradicts().is_err() {
            return false;
        }
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n {
            if (self.white[i] | self.black[i]) != (1 << m) - 1 {
                return false;
            }
        }
        true
    }
    pub fn contradicts(&self) -> Result<(), ()> {
        self.check_color_of_capes()?;
        self.check_capes()?;
        self.check_2x2()?;
        self.check_cape_numbers()?;
        self.check_connected()?;
        Ok(())
    }
    fn check_color_of_capes(&self) -> Result<(), ()> {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n {
            for j in 0..m {
                if self.init[i][j] != Square::Blank {
                    if (self.black[i] & 1 << j) != 0 {
                        return Err(());
                    }
                }
            }
        }
        Ok(())
    }
    fn check_capes(&self) -> Result<(), ()> {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n {
            for j in 0..m {
                if (self.white[i] & 1 << j) == 0 {
                    continue;
                }
                let mut num_nonblack = 0;
                let mut num_white = 0;
                if i > 0 {
                    if (self.black[i - 1] & 1 << j) == 0 {
                        num_nonblack += 1;
                    }
                    if (self.white[i - 1] & 1 << j) != 0 {
                        num_white += 1;
                    }
                }
                if i < n - 1 {
                    if (self.black[i + 1] & 1 << j) == 0 {
                        num_nonblack += 1;
                    }
                    if (self.white[i + 1] & 1 << j) != 0 {
                        num_white += 1;
                    }
                }
                let mask = 5 << j >> 1;
                num_nonblack += (!self.black[i] & mask).count_ones();
                num_white += (self.white[i] & mask).count_ones();
                if self.init[i][j] != Square::Blank {
                    if num_white >= 2 || num_nonblack == 0 {
                        return Err(());
                    }
                } else {
                    if num_nonblack <= 1 {
                        return Err(());
                    }
                }
            }
        }
        Ok(())
    }
    fn check_2x2(&self) -> Result<(), ()> {
        let n = self.init.len();
        for i in 0..n - 1 {
            let white = self.white[i] & self.white[i + 1];
            let black = self.black[i] & self.black[i + 1];
            if (white & white.wrapping_shl(1)) != 0 {
                return Err(());
            }
            if (black & black.wrapping_shl(1)) != 0 {
                return Err(());
            }
        }
        Ok(())
    }
    fn check_cape_numbers(&self) -> Result<(), ()> {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n {
            for j in 0..m {
                let num = match self.init[i][j] {
                    Square::Blank => continue,
                    Square::Number(x) => x.get() as usize,
                };
                if num == 1 {
                    continue;
                }
                let mut ok = 0;
                if i >= num - 1 {
                    if (i + 1 - num..i).all(|k| (self.black[k] & 1 << j) == 0)
                        && (i <= num - 1 || (self.white[i - num] & 1 << j) == 0)
                    {
                        ok += 1;
                    }
                }
                if i + num <= n {
                    if (i + 1..i + num).all(|k| (self.black[k] & 1 << j) == 0)
                        && (i + num >= n || (self.white[i + num] & 1 << j) == 0)
                    {
                        ok += 1;
                    }
                }
                if j >= num - 1 {
                    if (self.black[i] & ((1 << j) - (1 << (j + 1 - num)))) == 0
                        && (j <= num - 1 || (self.white[i] & 1 << (j - num)) == 0)
                    {
                        ok += 1;
                    }
                }
                if j + num <= m {
                    if (self.black[i] & ((1 << (j + num)) - (1 << (j + 1)))) == 0
                        && (j + num >= m || (self.white[i] & 1 << (j + num)) == 0)
                    {
                        ok += 1;
                    }
                }
                if ok == 0 {
                    return Err(());
                }
            }
        }
        Ok(())
    }
    fn check_connected(&self) -> Result<(), ()> {
        let n = self.init.len();
        let m = self.init[0].len();
        let mut uf = nurimisaki_solver::uf::UnionFind::new(n * m);
        for i in 0..n - 1 {
            let nonblack = !(self.black[i] | self.black[i + 1]);
            for j in 0..m {
                if (nonblack & 1 << j) != 0 {
                    let v = i * m + j;
                    uf.unite(v, v + m);
                }
            }
        }
        for i in 0..n {
            let nonblack = !self.black[i];
            for j in 0..m - 1 {
                if (nonblack & 3 << j) == (3 << j) {
                    let v = i * m + j;
                    uf.unite(v, v + 1);
                }
            }
        }
        let mut seen = vec![];
        for i in 0..n {
            for j in 0..m {
                if (self.white[i] & 1 << j) != 0 {
                    seen.push(uf.root(i * m + j));
                }
            }
        }
        seen.dedup();
        if seen.len() >= 2 {
            Err(())
        } else {
            Ok(())
        }
    }
    pub fn search(&mut self, stat: &mut Stat) -> bool {
        stat.num_call += 1;
        if self.finished() {
            return true;
        }
        if self.contradicts().is_err() {
            return false;
        }
        let n = self.init.len();
        let m = self.init[0].len();
        // very naive search
        // TODO:
        // search from capes
        for i in 0..n {
            let white = self.white[i];
            let black = self.black[i];
            let occupied = white | black;
            for j in 0..m {
                if (occupied & 1 << j) != 0 {
                    continue;
                }
                self.white[i] |= 1 << j;
                if self.search(stat) {
                    return true;
                }
                self.white[i] ^= 1 << j;
                self.black[i] |= 1 << j;
                if self.search(stat) {
                    return true;
                }
                self.black[i] ^= 1 << j;
                return false;
            }
        }
        false
    }
}

impl core::fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.init.len();
        let m = self.init[0].len();
        for i in 0..n {
            for j in 0..m {
                match self.init[i][j] {
                    Square::Blank => {
                        if (self.black[i] & 1 << j) != 0 {
                            f.write_str("*")?;
                        } else if (self.white[i] & 1 << j) != 0 {
                            f.write_str(".")?;
                        } else {
                            f.write_str("_")?;
                        }
                    }
                    Square::Number(num) => {
                        let num = num.get();
                        if num == 1 {
                            f.write_str("O")?;
                        } else {
                            write!(f, "{}", num)?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    let board = nurimisaki_solver::examples::example2();
    let mut board = Board::new(&board);
    println!("{}", board);
    println!("{:?}", board.contradicts());
    let mut stat = Stat::default();
    let result = board.search(&mut stat);
    println!("result = {}", result);
    println!("{}", board);
    // stat = Stat { num_call: 6999113 }
    println!("stat = {:?}", stat);
}
