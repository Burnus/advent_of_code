pub fn run() -> (usize, usize) {
    let first = next_valid_input(usize::MAX, false);
    let second = next_valid_input(usize::MIN, true);
    (first, second)
}

fn next_valid_input(previous: usize, ascending: bool) -> usize {
    let mut curr = if ascending {
        previous+1
    } else {
        previous-1
    };
    // We only need to guess 6 digits and can derive the rest from them, as shown below
    curr = curr.clamp(111_111, 999_999);
    let mut digits: Vec<isize>;
    loop {
        digits = Vec::new();
        let mut rest = curr;
        while rest > 0 {
            let digit = rest % 10;
            if digit == 0 {
                if ascending {
                    curr += 10_usize.pow(digits.len() as u32);
                } else {
                    curr -= 10_usize.pow(digits.len() as u32);
                }
                digits = Vec::new();
                rest = curr;
            } else {
                digits.push(digit as isize);
                rest /= 10;
            }
        }
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
        if other_digits.iter().all(|d| (1..10).contains(d)) {
            return 10_usize.pow(13) * digits[0] as usize +
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
                                     other_digits[5] as usize;
        } else {
            if ascending {
                curr += 1;
            } else {
                curr -= 1;
            }
            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge() {
        assert_eq!(run(), (29991993698469, 14691271141118));
    }
}
