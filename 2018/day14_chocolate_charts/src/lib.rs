pub fn run(input: &str) -> (usize, usize) {
    let target = input.parse::<usize>().unwrap();
    // let target_len = target.ilog10() as usize + 1;
    let target_len = input.chars().count();
    let mut scores = vec![3, 7, 1, 0];
    let mut elf_0 = 0;
    let mut elf_1 = 1;
    let mut second = 0;
    while scores.len() <= target + 10 || second == 0 {
        let combined = scores[elf_0] + scores[elf_1];
        if combined > 9 {
            scores.push(1);
        }
        scores.push(combined % 10);
        elf_0 += 1 + scores[elf_0];
        elf_0 %= scores.len();
        elf_1 += 1 + scores[elf_1];
        elf_1 %= scores.len();
        if second == 0 && scores.len() > target_len + 1 {
             if let Some(starting_idx) = (scores.len()-target_len-2..).take(2).find(|i| scores[*i..*i+target_len].iter()
                                                                            .enumerate()
                                                                            .map(|(idx, digit)| 10_isize.pow((target_len-idx-1) as u32) as usize * digit)
                                                                            .sum::<usize>() == target) {
                second = starting_idx;
             }
        }
    }
    let first = scores[target..target+10].iter().enumerate().map(|(idx, digit)| 10_isize.pow(9-idx as u32) as usize * digit).sum();
    (first, second)
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
        let expected = [
                (124515891, 9),
                (5158916779, 13),
                (9251071085, 48),
                (3910137144, 9),
                (1121413115, 5),
                (7541291229, 18),
                (5131221087, 2018),
            ];
        for (idx, input ) in sample_input.lines().enumerate() {
            assert_eq!(run(input), expected[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1413131339, 0));
    }
}
