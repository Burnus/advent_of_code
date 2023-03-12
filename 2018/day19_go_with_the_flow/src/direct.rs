pub fn run(first: isize) -> isize {
    let mut a = first;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut nip = 0;
    let mut f = 0;

    // nip += 17;
    c += 2; // nip += 1;
    c *= 2; // nip += 1;
    c *= 19; // nip += 1;
    c *= 11; // nip += 1;
    f += 2; // nip += 1;
    f *= 22; // nip += 1;
    f += 18; // nip += 1;
    c += f;  // nip += 1;
    match a {
        0 => {
                // b = c+1;
                // d = c + 1;
                (1..=c).filter(|b| c % b == 0).sum::<isize>()
                // f = 1;
                // nip = 16;
                // return a;
            }
        1 => {
                // f = 27 + d;
                // f *= 28;
                // f += 29;
                // f *= 30;
                // f *= 14;
                // f *= 32;
                // c += f;
                c += (27*28+29)*30*14*32;
                (1..=c).filter(|b| c % b == 0).sum::<isize>()
            },
        _ => panic!("Unexpected value in a: {a}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(run(0), 1350);
    }

    #[test]
    fn second() {
        assert_eq!(run(1), 15844608);
    }
}
