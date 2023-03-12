pub fn run(input: &str) -> (usize, usize) {
    let first = input.lines().filter(|t| check_trinangle(t)).count();
    let collected_lines: Vec<_> = input.lines().map(|l| l.split_whitespace().collect::<Vec<_>>()).collect();
    let second = collected_lines.chunks(3).map(|lines| {
        (0..3).filter(|i| {
            let (a, b, c): (usize, usize, usize) = ( lines[0][*i].parse().unwrap(), lines[1][*i].parse().unwrap(), lines[2][*i].parse().unwrap() );
            let max = a.max(b).max(c);
            a+b>max && b+c>max && a+c>max
        }).count()
    }).sum();
    (first, second)
}

fn check_trinangle(line: &str) -> bool {
    let components: Vec<_> = line.split_whitespace().collect();
    assert_eq!(components.len(), 3);
    let (a, b, c): (usize, usize, usize) = ( components[0].parse().unwrap(), components[1].parse().unwrap(), components[2].parse().unwrap() );
    let max = a.max(b).max(c);
    a+b>max && b+c>max && a+c>max
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1050, 1921));
    }
}
