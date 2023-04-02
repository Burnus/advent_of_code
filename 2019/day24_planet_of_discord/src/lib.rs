use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum BugState {
    Bug,
    Empty,
}

impl BugState {
    fn other(&self) -> Self {
        match self {
            Self::Bug => Self::Empty,
            Self::Empty => Self::Bug,
        }
    }
}

#[derive(Clone, Copy)]
struct Grid {
    bugs: [[BugState; 5]; 5],
}

impl TryFrom<&str> for Grid {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut bugs = [[BugState::Empty; 5]; 5];
        for (y, row) in value.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => bugs[y][x] = BugState::Bug,
                    _ => return Err(ParseError::LineMalformed(format!("Unexpected token: {c}"))),
                }
            }
        }
        Ok(Self { bugs })
    }
}

impl Grid {
    fn empty() -> Self {
        Self { bugs: [[BugState::Empty; 5]; 5] }
    }

    fn bugs_adjacent_to(&self, (x, y): (usize, usize)) -> usize {
        [(1, 0), (0, 1), (2, 1), (1, 2)].iter().filter(|(dx, dy)| (1..=5).contains(&(x+dx)) && (1..=5).contains(&(y+dy)) && self.bugs[y+dy-1][x+dx-1] == BugState::Bug).count()
    }

    fn next_minute(&mut self) {
        let mut next = self.bugs;
        for y in 0..5 {
            for x in 0..5 {
                next[y][x] = match self.bugs_adjacent_to((x, y)) {
                    1 => BugState::Bug,
                    2 => self.bugs[y][x].other(),
                    _ => BugState::Empty,
                }
            }
        }
        std::mem::swap(&mut next, &mut self.bugs);
    }

    fn biodiversity_rating(&self) -> usize {
        self.bugs.iter().enumerate().map(|(y, row)| row.iter().enumerate().map(|(x, state)| if state == &BugState::Bug { 2_usize.pow(5*y as u32 + x as u32) } else { 0 }).sum::<usize>()).sum()
    }

    fn print(&self) -> String {
        self.bugs.iter().map(|row| row.iter().map(|state| if state == &BugState::Bug { '#' } else { '.' }).chain(['\n'].into_iter()).collect::<String>()).collect()
    }
}

struct Grid3D {
    grids: [Grid; 401],
}

impl TryFrom<&str> for Grid3D {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grids = [Grid::empty(); 401];
        grids[200] = Grid::try_from(value)?;
        Ok(Self { grids, })
    }
}

impl Grid3D {
    fn bugs_adjacent_to(&self, (x, y, z): (usize, usize, usize)) -> usize {
        match (x, y, z) {
            (2, 2, _) => 0,
            (0, 0, 0) => [(0, 1, 0), (1, 0, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (0, 0, z) => [(0, 1, z), (1, 0, z), (2, 1, z-1), (1, 2, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (0, 4, 0) => [(0, 3, 0), (1, 4, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (0, 4, z) => [(0, 3, z), (1, 4, z), (1, 2, z-1), (2, 3, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (0, y, 0) => [(0, y-1, 0), (1, y, 0), (0, y+1, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (0, y, z) => [(0, y-1, z), (1, y, z), (0, y+1, z), (1, 2, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (4, 0, 0) => [(3, 0, 0), (4, 1, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (4, 0, z) => [(3, 0, z), (4, 1, z), (2, 1, z-1), (3, 2, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (4, 4, 0) => [(4, 3, 0), (3, 4, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (4, 4, z) => [(4, 3, z), (3, 4, z), (2, 3, z-1), (3, 2, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (4, y, 0) => [(4, y-1, 0), (3, y, 0), (4, y+1, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (4, y, z) => [(4, y-1, z), (3, y, z), (4, y+1, z), (3, 2, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (x, 0, 0) => [(x-1, 0, 0), (x, 1, 0), (x+1, 0, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (x, 0, z) => [(x-1, 0, z), (x, 1, z), (x+1, 0, z), (2, 1, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (x, 4, 0) => [(x-1, 4, 0), (x, 3, 0), (x+1, 4, 0)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (x, 4, z) => [(x-1, 4, z), (x, 3, z), (x+1, 4, z), (2, 3, z-1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (1, 2, 400) => [(1, 1, 400), (0, 2, 400), (1, 3, 400)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (2, 1, 400) => [(1, 1, 400), (2, 0, 400), (3, 1, 400)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (2, 3, 400) => [(1, 3, 400), (2, 4, 400), (3, 3, 400)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (3, 2, 400) => [(3, 1, 400), (4, 2, 400), (3, 3, 400)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (1, 2, z) => [(1, 1, z), (0, 2, z), (1, 3, z), (0, 0, z+1), (0, 1, z+1), (0, 2, z+1), (0, 3, z+1), (0, 4, z+1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (2, 1, z) => [(1, 1, z), (2, 0, z), (3, 1, z), (0, 0, z+1), (1, 0, z+1), (2, 0, z+1), (3, 0, z+1), (4, 0, z+1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (2, 3, z) => [(1, 3, z), (2, 4, z), (3, 3, z), (0, 4, z+1), (1, 4, z+1), (2, 4, z+1), (3, 4, z+1), (4, 4, z+1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (3, 2, z) => [(3, 1, z), (4, 2, z), (3, 3, z), (4, 0, z+1), (4, 1, z+1), (4, 2, z+1), (4, 3, z+1), (4, 4, z+1)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
            (x, y, z) => [(x-1, y, z), (x, y-1, z), (x+1, y, z), (x, y+1, z)].iter().filter(|(x, y, z)| self.grids[*z].bugs[*y][*x] == BugState::Bug).count(),
        }
    }

    fn next_minute(&mut self) {
        let mut next = self.grids;
        for z in 0..401 {
            for y in 0..5 {
                for x in 0..5 {
                    next[z].bugs[y][x] = match self.bugs_adjacent_to((x, y, z)) {
                        1 => BugState::Bug,
                        2 => self.grids[z].bugs[y][x].other(),
                        _ => BugState::Empty,
                    }
                }
            }
        }
        std::mem::swap(&mut next, &mut self.grids);
    }
}

pub fn run_1(input: &str) -> Result<usize, ParseError> {
    let mut grid = Grid::try_from(input)?;
    let mut previous = HashSet::from([grid.biodiversity_rating()]);
    loop {
        grid.next_minute();
        let this = grid.biodiversity_rating();
        if previous.contains(&this) {
            return Ok(this);
        }
        previous.insert(this);
    }
}

pub fn run_2(input: &str, turns: usize) -> Result<usize, ParseError> {
    let mut grid = Grid3D::try_from(input)?;
    for _ in 0..turns {
        grid.next_minute();
    }
    // for level in 195..=205 {
    //     println!("{}", grid.grids[level].print());
    // }
    Ok(grid.grids.iter().map(|level| level.bugs.iter().map(|row| row.iter().filter(|state| state == &&BugState::Bug).count()).sum::<usize>()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn biodiversity_rating_sample() {
        let mut grid = Grid{ bugs: [[BugState::Empty; 5]; 5], };
        grid.bugs[3][0] = BugState::Bug;
        grid.bugs[4][1] = BugState::Bug;
        assert_eq!(grid.biodiversity_rating(), 2129920);
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        let mut grid = Grid::try_from(&sample_input[..]).unwrap();
        let expected_states = [
"....#
#..#.
#..##
..#..
#....
",
"#..#.
####.
###.#
##.##
.##..
",
"#####
....#
....#
...#.
#.###
",
"#....
####.
...##
#.##.
.##.#
",
"####.
....#
##..#
.....
##...
",
];
        for expected in expected_states {
            let actual = grid.print();
            assert_eq!(&actual[..], expected);
            grid.next_minute();
        }

        assert_eq!(run_2(&sample_input, 10), Ok(99))
        // assert_eq!(run(&sample_input), Ok((0, 0)));
    }

    #[test]
    fn test_recursive_next() {
        let initial = 
"....#
#..#.
#..##
..#..
#....";
        let mut grid = Grid3D::try_from(initial).unwrap();
        let expected_1 = [
".....
..#..
...#.
..#..
.....
",
"#..#.
####.
##..#
##.##
.##..
",
"....#
....#
....#
....#
#####
",
];
        let expected_2 = [
"..#..
.#.#.
....#
.#.#.
..#..
",
".....
.....
.....
...#.
.....
",
"####.
#..#.
#..#.
####.
.....
",
];
        let expected_3 = [
".....
..#..
...#.
..#..
.....
",

".#.#.
#...#
.#...
#...#
.#.#.
",

".....
.....
...#.
..#.#
...#.
",

"....#
.##.#
.#..#
....#
####.
",

];
        grid.next_minute();
        for (level, bugs) in expected_1.iter().enumerate() {
            assert_eq!(grid.grids[199+level].print(), bugs.to_string());
        }

        grid.next_minute();
        for (level, bugs) in expected_2.iter().enumerate() {
            assert_eq!(grid.grids[199+level].print(), bugs.to_string());
        }

        grid.next_minute();
        for (level, bugs) in expected_3.iter().enumerate() {
            assert_eq!(grid.grids[198+level].print(), bugs.to_string());
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run_1(&challenge_input), Ok(28781019));
        assert_eq!(run_2(&challenge_input, 200), Ok(1939));
    }
}
