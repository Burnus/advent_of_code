use std::num::ParseIntError;

const MODULUS: usize = 20201227;

// The algorithm we are trying to break here is a Diffie-Hellman key exchange. Fortunately, we are
// operating with pretty low numbers (MODULUS is only about 2.pow(25)), so we'll be fine
// bruteforcing it.
pub fn run(input: &str) -> Result<usize, ParseIntError> {
    let public: Vec<_> = input.lines().map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let loop_size = guess_loop_size(public[0]);
    let res = transform(public[1], loop_size);
    Ok(res)
}

// The Transformation is equivalent to `subject_number.pow(loop_size) % MODULO`. We can do this
// efficiently using exponentiation by squaring.
fn transform(subject_number: usize, loop_size: usize) -> usize {
    if loop_size == 0 {
        1
    } else if loop_size % 2 == 0 {
        transform((subject_number * subject_number) % MODULUS, loop_size/2)
    } else {
        (subject_number * transform((subject_number * subject_number) % MODULUS, loop_size/2)) % MODULUS
    }
}

// We are supposed to compute the discrete logarithm of `public` for base 7. This is generally a
// computationally hard problem, but there might be an efficient algorithm for these specific
// inputs. However, since we are using a group size of only about 2.pow(25), we may as well just
// loop until we find the observed value.
fn guess_loop_size(public: usize) -> usize {
    let subject_number = 7;
    let mut res = 1;
    let mut loop_size = 0;
    while res != public {
        loop_size += 1;
        res *= subject_number;
        res %= MODULUS;
    }
    loop_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok(14897079));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok(16457981));
    }
}
