#[derive(Clone)]
struct Star {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl From<&str> for Star {
    fn from(value: &str) -> Self {
        let components: Vec<_> = value.split(&['<', ',', '>'][..]).collect();
        // dbg!(&components);
        assert_eq!(components.len(), 7);
        Self {
            position: (components[1].trim().parse().unwrap(), components[2].trim().parse().unwrap()),
            velocity: (components[4].trim().parse().unwrap(), components[5].trim().parse().unwrap()),
        }
    }
}

impl Star {
    fn mov(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

pub fn run(input: &str) -> (String, usize) {
    let mut stars: Vec<_> = input.lines().map(Star::from).collect();
    let mut next_stars = stars.clone();
    let mut bbox = get_bbox(&stars);
    let mut second = 0;
    loop {
        for star in next_stars.iter_mut() {
            star.mov();
        }
        let next_bbox = get_bbox(&next_stars);
        if bbox_size(bbox) < bbox_size(next_bbox) {
            break;
        }
        second += 1;
        stars = next_stars.clone();
        bbox = next_bbox;
    }
    // println!("\n\n");
    // let res = print_stars(&stars, bbox);
    // for line in res.lines() {
    //     println!("{line}");
    // }
    let first = print_stars(&stars, bbox);
    // let second = 0;
    // for line in first.lines() {
    //     println!("{line}");
    // }
    (first, second)
}

fn bbox_size(bbox: ((isize, isize), (isize, isize))) -> isize {
    (bbox.1.0 - bbox.0.0) * (bbox.1.1 - bbox.0.1)
}

fn get_bbox(stars: &[Star]) -> ((isize, isize), (isize, isize)) {
    (
        (
            stars.iter().map(|star| star.position.0).min().unwrap(),
            stars.iter().map(|star| star.position.1).min().unwrap(),
        ),
        (
            stars.iter().map(|star| star.position.0).max().unwrap(),
            stars.iter().map(|star| star.position.1).max().unwrap(),
        )
    )
}

fn print_stars(stars: &[Star], ((min_x, min_y), (max_x, max_y)): ((isize, isize), (isize, isize))) -> String {
    (min_y..=max_y).map(|y| 
        (min_x..=max_x).map(|x| {
                if stars.iter().any(|star| star.position == (x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                }
            ).chain(['\n'].into_iter())
            .collect::<String>())
        .collect()
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
        let expected = r#"
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
"#;
        assert_eq!(run(&sample_input), (expected[1..].to_string(), 3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let expected = r#"
######...####...#....#..#....#.....###..#..........###..######
#.......#....#..#...#...#....#......#...#...........#...#.....
#.......#.......#..#.....#..#.......#...#...........#...#.....
#.......#.......#.#......#..#.......#...#...........#...#.....
#####...#.......##........##........#...#...........#...#####.
#.......#.......##........##........#...#...........#...#.....
#.......#.......#.#......#..#.......#...#...........#...#.....
#.......#.......#..#.....#..#...#...#...#.......#...#...#.....
#.......#....#..#...#...#....#..#...#...#.......#...#...#.....
######...####...#....#..#....#...###....######...###....#.....
"#;
        assert_eq!(run(&challenge_input), (expected[1..].to_string(), 10880));
    }
}
