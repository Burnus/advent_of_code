pub fn run() -> (usize, usize) {
    let first = next_valid_input(1_000_000, false);
    let second = next_valid_input(111_110, true);
    (first, second)
}

fn next_valid_input(previous: usize, ascending: bool) -> usize {
    let mut curr = if ascending {
        previous+1
    } else {
        previous-1
    };
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
// [0]=[13]-7
// [1]=[12]+3
// [2]=[11]+5
// [3]=9
// [4]=1
// [5]=[10]+1
// [6]=[7]+6
// [8]=[9]-3
//
// free: 13, 12, 11, 10, 9, 7
// [13] = 0
// [12] = 1
// [11] = 2
// [10] = 3
// [9] = 4
// [8] = 5
// [7] = 6

        let seven = digits.pop().unwrap();
        let next = digits[4] - 3;
        if (1..10).contains(&next) {
            digits.push(next);
        } else {
            if ascending {
                curr += 1;
            } else {
                curr -= 1;
            }
            continue;
        }
        digits.push(seven);
        let next = digits[6] + 6;
        if (1..10).contains(&next) {
            digits.push(next);
        } else {
            if ascending {
                curr += 1;
            } else {
                curr -= 1;
            }
            continue;
        }
        let next = digits[3] + 1;
        if (1..10).contains(&next) {
            digits.push(next);
        } else {
            if ascending {
                curr += 1;
            } else {
                curr -= 1;
            }
            continue;
        }
        digits.push(1);
        digits.push(9);
        let next = digits[2] + 5;
        if (1..10).contains(&next) {
            digits.push(next);
        } else {
            if ascending {
                curr += 1;
            } else {
                curr -= 1;
            }
            continue;
        }
        let next = digits[1] + 3;
        if (1..10).contains(&next) {
            digits.push(next);
        } else {
            if ascending {
                curr += 1;
            } else {
                curr -= 1;
            }
            continue;
        }
        let next = digits[0] - 7;
        if (1..10).contains(&next) {
            digits.push(next);
            break;
        } else if ascending {
            curr += 1;
        } else {
            curr -= 1;
        }
    }
    digits.iter().rev().fold(0, |acc, cur| 10*acc+ *cur as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge() {
        assert_eq!(run(), (29991993698469, 14691271141118));
    }
}
