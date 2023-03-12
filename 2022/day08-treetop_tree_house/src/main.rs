use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn get_visibility(forest: &[Vec<u8>]) -> usize {
    let rows = forest.len();
    let cols = forest[0].len();

    let mut visible = (rows+cols-2)*2; // all border trees are already visible and have a scenic score of 0.
    for row in 1..forest.len()-1 {
        for col in 1..forest[0].len()-1 {
            let tree_height = forest[row][col];

            if forest[row].iter()
                    .take(col)
                    .max()
                    .unwrap() < &tree_height ||
                forest[row].iter()
                    .skip(col+1)
                    .max()
                    .unwrap() < &tree_height ||
                forest.iter()
                    .take(row)
                    .map(|row| row[col])
                    .max()
                    .unwrap() < tree_height ||
                forest.iter()
                    .skip(row+1)
                    .map(|row| row[col])
                    .max()
                    .unwrap() < tree_height {
                        visible += 1;
                    }
        }
    }
    visible
}

fn get_scenic_score(forest: &[Vec<u8>]) -> usize {
    let rows = forest.len();
    let cols = forest[0].len();

    let mut highest_scenic_score = 0;
    for row in 1..forest.len()-1 {
        for col in 1..forest[0].len()-1 {
            let tree_height = forest[row][col];

            let mut scenic_score = 1;
            let mut this_factor = 0;
            for this_col in (0..col).rev() {
                this_factor += 1;
                if forest[row][this_col] >= tree_height { break; }
            }
            scenic_score *= this_factor;
            this_factor = 0;

            for this_col in col+1..cols {
                this_factor += 1;
                if forest[row][this_col] >= tree_height { break; }
            }
            scenic_score *= this_factor;
            this_factor = 0;

            for this_row in (0..row).rev() {
                this_factor += 1;
                if forest[this_row][col] >= tree_height { break; }
            }
            scenic_score *= this_factor;
            this_factor = 0;

            for this_row in row+1..rows {
                this_factor += 1;
                if forest[this_row][col] >= tree_height { break; }
            }
            scenic_score *= this_factor;

            highest_scenic_score = highest_scenic_score.max(scenic_score);
        }
    }
    highest_scenic_score
}

fn main() {
    let trees = read_file("input");
    let forest: Vec<Vec<u8>> = trees.lines()
        .map(|row| row.bytes()
             .map(|c| c as u8 - b'0')
             .collect())
        .collect();

    let visible = get_visibility(&forest);
    let highest_scenic_score = get_scenic_score(&forest);

    println!("{visible} trees are visible from at least one edge.");
    println!("The highest scenic score is {highest_scenic_score}");
}

#[test]
fn sample_input() {
    let trees = read_file("tests/sample_input");
    let forest: Vec<Vec<_>> = trees.lines().map(|r| r.bytes().map(|c| c as u8 - b'0').collect()).collect();
    assert_eq!(get_visibility(&forest), 21);
    assert_eq!(get_scenic_score(&forest), 8);

}

#[test]
fn challenge_input() {
    let trees = read_file("tests/input");
    let forest: Vec<Vec<_>> = trees.lines().map(|r| r.bytes().map(|c| c as u8 - b'0').collect()).collect();
    assert_eq!(get_visibility(&forest), 1695);
    assert_eq!(get_scenic_score(&forest), 287040);

}
