pub fn run() -> (usize, usize) {
    let first = next_valid_input(usize::MAX, false);
    let second = next_valid_input(usize::MIN, true);
    (first, second)
}

/// Takes a 6 digit seed (since we can somewhat freely chose 6 of the 14 digits) and a bool
/// indicating wether we want the next higher (true), or lower (false) valid input. Produces said
/// input. If seed is not 6 digits long, it will be clamped to be (so next_valid_input(0, true)
/// will produce the lowest valid input, and next_valid_input(1_000_000, false) will produce the
/// highest).
fn next_valid_input(seed: usize, ascending: bool) -> usize {
    // Make sure we have a 6 digit seed to begin with (containing no 0s, as per spec)
    let curr = match ascending {
        true => (seed+1).clamp(111_111, 999_999),
        false => (seed-1).clamp(111_111, 999_999),
    };

    // The seed digits need to fall into the following ranges, so we can add/substract the
    // calculated offsets for the other digits later. The digits are considered right to left,
    // which makes it easier for me to splice them into an array and manipulate them accordingly.
    let digit_ranges = [
        (1, 6),
        (7, 9),
        (2, 9),
        (6, 9),
        (4, 9),
        (1, 2),
    ];
    let mut digits: Vec<isize>;
    digits = Vec::new();
    let mut rest = curr;
    while rest > 0 {
        let digit = rest % 10;
        digits.push(digit as isize);
        rest /= 10;
    }
    if ascending {
        for idx in 0..digits.len() {
            let digit = digits[idx];
            digits[idx] = digit.max(digit_ranges[idx].0);
            if digit > digit_ranges[idx].1 {
                digits[idx] = digit_ranges[idx].1;
                for i in 0..idx {
                    digits[i] = digit_ranges[i].0;
                }
            }
        }
    } else {
        for idx in 0..digits.len() {
            let digit = digits[idx];
            digits[idx] = digit.min(digit_ranges[idx].1);
            if digit < digit_ranges[idx].0 {
                digits[idx] = digit_ranges[idx].0;
                for i in 0..idx {
                    digits[i] = digit_ranges[i].1;
                }
            }
        }
    }
    digits.reverse();
    // From carefully observing the input code, we know that half the digits can be linearly
    // derived from the other half, using the following conversion (where [n] denotes the nth
    // most significant digit of the number):
    // [3]  = 9
    // [4]  = 1
    // [7]  = [6] - 6
    // [9]  = [8] + 3
    // [10] = [5] - 1
    // [11] = [2] - 5
    // [12] = [1] - 3
    // [13] = [0] + 7
    // 
    // Since we haven't pushed [3] and [4] to the digits Vec yet, the indices of the digits to
    // derive [7] to [10] from need to be reduced by 2 though. Also, we need to be careful to
    // swap [7] and [8]. That won't change ordering though, because [7] is determined by [6],
    // which already dominates [8].

    let other_digits = [
        digits[4]-6,
        digits[5]+3,
        digits[3]-1,
        digits[2]-5,
        digits[1]-3,
        digits[0]+7,
    ];
    10_usize.pow(13) * digits[0] as usize +
        10_usize.pow(12) * digits[1] as usize +
        10_usize.pow(11) * digits[2] as usize +
        10_usize.pow(10) * 9 +
        10_usize.pow(9) +
        10_usize.pow(8) * digits[3] as usize +
        10_usize.pow(7) * digits[4] as usize +
        10_usize.pow(6) * other_digits[0] as usize +
        10_usize.pow(5) * digits[5] as usize +
        10_usize.pow(4) * other_digits[1] as usize +
        10_usize.pow(3) * other_digits[2] as usize +
        10_usize.pow(2) * other_digits[3] as usize +
        10              * other_digits[4] as usize +
        other_digits[5] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge() {
        assert_eq!(run(), (29991993698469, 14691271141118));
    }
}
