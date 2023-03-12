use std::collections::HashMap;


pub fn run(input: &str) -> (usize, usize) {
    let mut log: Vec<_> = input.lines().collect();
    log.sort();
    let guards = read_log(&log);
    let most_asleep = guards.iter().max_by_key(|(_id, minutes)|minutes.iter().sum::<usize>()).map(|(id, _m)| id).unwrap();
    let sleepiest_minute = guards.get(most_asleep)
                            .unwrap()
                            .iter()
                            .enumerate()
                            .max_by_key(|(_min, time)| *time)
                            .map(|(min, _time)| min)
                            .unwrap();
    let first = most_asleep * sleepiest_minute;
    let (guard, sleepiest_minute_total) = guards.iter()
                            .max_by_key(|(_id, minutes)| minutes.iter().max())
                            .map(|(id, minutes)| (id, minutes.iter()
                                                  .enumerate()
                                                  .max_by_key(|(_min, asleep)| *asleep)
                                                  .map(|(min, _asleep)| min)
                                                  .unwrap()))
                            .unwrap();
    let second = guard * sleepiest_minute_total;
    (first, second)
}

fn read_log(log: &[&str]) -> HashMap<usize, [usize; 61]> {
    let mut guards = HashMap::new();
    let mut id = 0;
    let mut asleep_since = 0;
    log.iter().for_each(|line| {
        let components: Vec<_> = line.split_whitespace().collect();
        match components[2] {
            "Guard" => {
                    id = components[3][1..].parse().unwrap();
                },
            "falls" => {
                    asleep_since = components[1][3..=4].parse().unwrap();
                },
            "wakes" => {
                    let awake: usize = components[1][3..=4].parse().unwrap();
                    if awake < asleep_since { panic!("Woke up too early: {line}"); }
                    let current = guards.entry(id).or_insert([0; 61]);
                    current.iter_mut().skip(asleep_since).take(awake - asleep_since).for_each(|m| *m += 1);
                },
            _ => panic!("Log line not recognized: {line}"),
        }
    });

    guards
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
        assert_eq!(run(&sample_input), (240, 4455));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (50558, 28198));
    }
}
