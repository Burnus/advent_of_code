#[derive(Debug)]
struct Screen {
    pixels: [[u8; 50]; 6],
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: [[0; 50]; 6],
        }
    }

    fn rect(&mut self, (x, y): (&str, &str)) {
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
        (0..x).for_each(|row| (0..y).for_each(|col| self.pixels[col][row] = 1));
    }

    fn rotate_row(&mut self, y: usize, by: usize) {
        let old_row = self.pixels[y];
        (0..50).for_each(|x| self.pixels[y][x] = old_row[(50 + x - by) % 50]);
    }

    fn rotate_column(&mut self, x: usize, by: usize) {
        let old_col: Vec<u8> = self.pixels.iter().map(|row| row[x]).collect();
        (0..6).for_each(|y| self.pixels[y][x] = old_col[(6 + y - by) % 6]);
    }

    fn perform(&mut self, line: &str) {
        let components: Vec<_> = line.trim().split(' ').collect();
        assert!((2..=5).contains(&components.len()));
        match (components[0], components[1]) {
            ("rect", dim) => self.rect(dim.split_once('x').unwrap()),
            ("rotate", "row") => self.rotate_row(components[2][2..].parse().unwrap(), components[4].parse().unwrap()),
            ("rotate", "column") => self.rotate_column(components[2][2..].parse().unwrap(), components[4].parse().unwrap()),
            _ => panic!("Unexpected Command: {line}"),
        }
    }

    fn render(&self) -> String {
        self.pixels.iter().map(|row| row.iter().map(|px| match px { 0=>" ", _=>"#",}).collect::<String>() + "\n").collect()
    }
}

pub fn run(input: &str) -> (usize, String) {
    let mut the_screen = Screen::new();
    input.lines().for_each(|line| the_screen.perform(line));
    let first = the_screen.pixels.into_iter().map(|row| row.into_iter().map(|pixel| pixel as usize).sum::<usize>()).sum();
    let second = the_screen.render();
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
        let mut the_screen = Screen::new();
        sample_input.lines().for_each(|line| the_screen.perform(line));
        assert_eq!(run(&sample_input), (6, 
"
    # #                                           
# #                                               
 #                                                
 #                                                
                                                  
                                                  
"[1..].to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (110, 
"
####   ## #  # ###  #  #  ##  ###  #    #   #  ## 
   #    # #  # #  # # #  #  # #  # #    #   #   # 
  #     # #### #  # ##   #    #  # #     # #    # 
 #      # #  # ###  # #  #    ###  #      #     # 
#    #  # #  # # #  # #  #  # #    #      #  #  # 
####  ##  #  # #  # #  #  ##  #    ####   #   ##  
"[1..].to_string()));
    }
}
