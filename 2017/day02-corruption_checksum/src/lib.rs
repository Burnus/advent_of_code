pub fn run(input: &str) -> (usize, usize) {
    let table: Vec<Vec<usize>> = input.lines().map(|line| line.split(' ').map(|i| i.parse::<usize>().unwrap()).collect()).collect();
    (sum_of_line_spans(&table), sum_of_divisible(&table))
}

fn sum_of_divisible(table: &[Vec<usize>]) -> usize {
    table.iter()
        .map(|row| (0..row.len())
                    .map(|l| (0..row.len())
                        .filter(|&r| l != r && row[l] % row[r] == 0)
                        .map(|r| row[l] / row[r])
                        .max().unwrap_or(0)
                    ).max().unwrap()
             )
        .sum()
}

fn sum_of_line_spans(table: &[Vec<usize>]) -> usize {
     table.iter()
         .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
         .sum()
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
        assert_eq!(run(&sample_input), (18, 9));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (45972, 326));
    }
}
