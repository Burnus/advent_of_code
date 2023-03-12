use std::collections::HashSet;

pub fn run() -> (usize, usize) {
    (first(), second())
}

fn first() -> usize {
    let mut a = 0;
    let mut d = a | 65536;
    a = 3798839;
    while d >= 1 {
        a += d & 255;
        a &= 16777215;
        a *= 65899;
        a &= 16777215;
        d /= 256;
    }
    a
}

fn second() -> usize {
    let mut seen = HashSet::new();
    let mut last = 0;
    let mut a = 0;
    loop {
        let mut d = a | 65536;
        a = 3798839;
        while d >= 1 {
            a += d & 255;
            a &= 16777215;
            a *= 65899;
            a &= 16777215;
            d /= 256;
        }
        if seen.contains(&a) {
            return last;
        } else {
            last = a;
            seen.insert(a);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge() {
        assert_eq!(run(), (1797184, 11011493));
    }
}
