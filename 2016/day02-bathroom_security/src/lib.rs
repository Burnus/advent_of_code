struct Digit {
    num: u8,
    al_num: char,
}

impl Digit {
    fn new() -> Self {
        Self {
            num: 5,
            al_num: '5',
        }
    }

    fn mov(&mut self, c: char) {
        match c {
            'U' => {
                    if self.num>3 { 
                        self.num -= 3;
                    }
                    self.al_num = match self.al_num as u8 {
                        b'3' => '1',
                        c if (b'6'..=b'8').contains(&c) => (c-4) as char,
                        c if (b'A'..=b'C').contains(&c) => (c-11) as char,
                        b'D' => 'B',
                        _ => self.al_num,
                    }
                },
            'D' => {
                    if self.num<7 { 
                        self.num += 3;
                    }
                    self.al_num = match self.al_num as u8 {
                        b'1' => '3',
                        c if (b'2'..=b'4').contains(&c) => (c+4) as char,
                        c if (b'6'..=b'8').contains(&c) => (c+11) as char,
                        b'B' => 'D',
                        _ => self.al_num,
                    }
                },
            'L' => {
                    if self.num%3 != 1 {
                        self.num -= 1;
                    }
                    self.al_num = match self.al_num as u8 {
                        c if (b'3'..=b'4').contains(&c) => (c-1) as char,
                        c if (b'6'..=b'9').contains(&c) => (c-1) as char,
                        c if (b'B'..=b'C').contains(&c) => (c-1) as char,
                        _ => self.al_num,
                    }
                },
            'R' => {
                    if self.num%3 > 0 {
                        self.num += 1;
                    }
                    self.al_num = match self.al_num as u8 {
                        c if (b'2'..=b'3').contains(&c) => (c+1) as char,
                        c if (b'5'..=b'8').contains(&c) => (c+1) as char,
                        c if (b'A'..=b'B').contains(&c) => (c+1) as char,
                        _ => self.al_num,
                    }
                },
            _ => ()
        }
    }
}

pub fn run(input: &str) -> (usize, String) {
    let mut first = 0;
    let mut second = String::new();
    let mut finger = Digit::new();
    input.lines().for_each(|line| {
        first *= 10;
        line.chars().for_each(|c| finger.mov(c));
        first += finger.num as usize;
        second += &(finger.al_num.to_string());
    });
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (1985, "5DB3".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (78293, "AC8C8".to_string()));
    }
}
