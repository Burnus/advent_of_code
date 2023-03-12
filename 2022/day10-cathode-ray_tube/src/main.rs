use std::fs;

struct Cpu {
    states: Vec<i32>,
}

impl Cpu {
    fn addx(&mut self, x: i32) {
        let old_state = *self.states.last().unwrap();
        self.states.push(old_state);
        self.states.push(old_state + x);
    }

    fn noop(&mut self) {
        let old_state = *self.states.last().unwrap();
        self.states.push(old_state);
    }

    fn parse(&mut self, instruction: &str) {
        match &instruction[0..4] {
            "noop" => self.noop(),
            "addx" => self.addx(instruction[5..].parse().unwrap()),
            _ => panic!("Unknown instruction"),
        }
    }

    fn get_rendering(&self) -> String {
        let mut rendering = String::new();
        (0..self.states.len()/40).for_each(|line_number| {
            if line_number != 0 {
                rendering.push('\n');
            }
            let mut this_line = String::new();
            (0..40).for_each(|col_number| {
                let clock_cycle = 40*line_number+col_number;
                if (clock_cycle as i32 % 40 - self.states[clock_cycle + 1]).abs() < 2 {
                    this_line += "#";
                } else {
                    this_line += ".";
                }
            });
            rendering.push_str(&this_line);
        });
        rendering
    }

    fn render(&self) {
        for line_number in 0..self.states.len()/40 {
            let mut line_string = String::new();
            for col_number in 0..40 {
                let clock_cycle = 40*line_number+col_number;
                if (clock_cycle as i32 % 40 - self.states[clock_cycle + 1]).abs() < 2 {
                    line_string += "#";
                } else {
                    line_string += ".";
                }
            }
            println!("{line_string}");
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn main() {
    //let program = read_file("sample_input");
    let program = read_file("input");

    let mut cpu = Cpu { states: vec![1, 1], };
    for instruction in program.lines() {
        cpu.parse(instruction);
    }
    let sum_of_relevant_strengths: i32 = [20, 60, 100, 140, 180, 220].iter()
        .map(|&i| i as i32 * cpu.states[i])
        .sum();

    println!("The relevant signal strengths sum up to {sum_of_relevant_strengths}.");
    cpu.render();
}

#[test]
fn sample_input() {
    let program = read_file("tests/sample_input");

    let mut cpu = Cpu { states: vec![1, 1], };
    for instruction in program.lines() {
        cpu.parse(instruction);
    }
    let sum_of_relevant_strengths: i32 = [20, 60, 100, 140, 180, 220].iter()
        .map(|&i| i as i32 * cpu.states[i])
        .sum();

    assert_eq!(sum_of_relevant_strengths, 13140);
    assert_eq!(cpu.get_rendering(), r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#);
}

#[test]
fn challenge_input() {
    let program = read_file("tests/input");

    let mut cpu = Cpu { states: vec![1, 1], };
    for instruction in program.lines() {
        cpu.parse(instruction);
    }
    let sum_of_relevant_strengths: i32 = [20, 60, 100, 140, 180, 220].iter()
        .map(|&i| i as i32 * cpu.states[i])
        .sum();

    assert_eq!(sum_of_relevant_strengths, 14720);
    assert_eq!(cpu.get_rendering(), r#"####.####.###..###..###..####.####.####.
#.......#.#..#.#..#.#..#.#.......#.#....
###....#..###..#..#.###..###....#..###..
#.....#...#..#.###..#..#.#.....#...#....
#....#....#..#.#....#..#.#....#....#....
#....####.###..#....###..#....####.#...."#);
}
