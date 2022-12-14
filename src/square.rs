use core::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Blank,
    Number(NonZeroU8), // 1 -> indeterminate circle, >=2 -> numbered circle
}

impl Square {
    pub fn new(a: u8) -> Self {
        if a == 0 {
            Square::Blank
        } else {
            Square::Number(unsafe { NonZeroU8::new_unchecked(a) })
        }
    }
}

/// Parses a URL like https://puzz.link/p?nurimisaki/9/9/.2zzzy
/// or https://puzz.link/p?nurimisaki_edit/9/9/.2zzzy.
pub fn parse_from_puzz_link(s: &str) -> Option<Vec<Vec<Square>>> {
    let s = if let Some(t) = s.strip_prefix("https://puzz.link/p?nurimisaki") {
        t
    } else {
        return None;
    };
    let s = if let Some(t) = s.strip_prefix("/") {
        t
    } else if let Some(t) = s.strip_prefix("_edit/") {
        t
    } else {
        return None;
    };
    let split: Vec<_> = s.split("/").collect();
    if split.len() != 3 {
        return None;
    }
    let m = split[0].parse::<usize>().ok()?;
    let n = split[1].parse::<usize>().ok()?;
    if n * m >= 10_000 || split[2].len() >= 10_000 {
        return None;
    }
    let mut data = vec![];
    let mut next_2 = false;
    let mut carry = 0;
    for c in split[2].chars() {
        if c == '.' {
            data.push(Square::new(1));
            continue;
        }
        if c >= 'g' && c <= 'z' {
            let len = (c as u8 - b'f') as usize;
            data.extend_from_slice(&vec![Square::Blank; len]);
            continue;
        }
        if c == '-' {
            next_2 = true;
            continue;
        }
        if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') {
            let dig = if c <= '9' {
                c as u8 - b'0'
            } else {
                c as u8 - b'a' + 10
            };
            if next_2 {
                carry = 16 * dig;
                next_2 = false;
            } else {
                data.push(Square::new(dig + carry));
                carry = 0;
            }
            continue;
        }
        return None;
    }
    if data.len() != n * m {
        return None;
    }
    let mut ret = vec![vec![Square::Blank; m]; n];
    for i in 0..n {
        ret[i].copy_from_slice(&data[i * m..i * m + m]);
    }
    Some(ret)
}
