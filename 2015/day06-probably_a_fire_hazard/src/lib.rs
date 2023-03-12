use std::fs::read_to_string;

enum Command { On, Off, Toggle }

fn read_file(name: &str) -> String {
    read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
}

fn perform(instruction: &str, target: &mut [Vec<u32>], op_mode: u8) {
    let components: Vec<&str> = instruction.split(' ').collect();
    let (command, tl, br) = match components.len() {
        4 => (Command::Toggle, components[1], components[3]),
        5 if components[1] == "on" => (Command::On, components[2], components[4]),
        5 if components[1] == "off" => (Command::Off, components[2], components[4]),
        _ => panic!("Unable to parse {instruction}"),
    };

    let (x_min, y_min) = tl.split_once(',').unwrap();
    let (x_max, y_max) = br.split_once(',').unwrap();
    let x_min = x_min.parse::<usize>().unwrap();
    let x_max = x_max.parse::<usize>().unwrap();
    let y_min = y_min.parse::<usize>().unwrap();
    let y_max = y_max.parse::<usize>().unwrap();

    target.iter_mut()
        .take(x_max+1)
        .skip(x_min)
        .for_each(|row| { row.iter_mut()
                        .take(y_max+1)
                        .skip(y_min)
                        .for_each(|cell| { 
                            let old_val = *cell;
                            if op_mode == 1 {
                                *cell = match command {
                                    Command::On => 1,
                                    Command::Off => 0,
                                    Command::Toggle => 1-old_val,
                                }; 
                            } else {
                                *cell = match command {
                                    Command::On => old_val + 1,
                                    Command::Off => old_val.saturating_sub(1),
                                    Command::Toggle => old_val + 2,
                                };
                            }
                        });
            }); 
}

pub fn run(input: &str) -> (u32, u32) {
    let mut lights_1 = vec![vec![0_u32; 1000]; 1000];
    let mut lights_2 = vec![vec![0_u32; 1000]; 1000];
    for instruction in input.lines() {
        perform(instruction, &mut lights_1, 1);
        perform(instruction, &mut lights_2, 2);
    }

    let first = lights_1.iter().flatten().sum();
    let second = lights_2.iter().flatten().sum();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (998996, 1_001_996));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (543903, 14687245));
    }
}
