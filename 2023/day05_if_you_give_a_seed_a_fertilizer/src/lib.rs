use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
    InputMalformed(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed(v) => write!(f, "Input is malformed: {v}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let steps: Vec<_> = input.split("\n\n").collect();
    if steps.len() < 2 {
        return Err(ParseError::InputMalformed(input));
    }
    let mut seeds: Vec<usize> = steps[0].split_whitespace().skip(1).map(|n| n.parse()).collect::<Result<Vec<_>, ParseIntError>>()?;
    if !seeds.len() % 2 == 0 {
        return Err(ParseError::LineMalformed(steps[0]));
    }
    let mut seed_ranges: Vec<_> = seeds.chunks(2).map(|c| (c[0], c[1])).collect();
    for step in steps.iter().skip(1) {
        let mut maps = Vec::new();

        for map in step.lines().skip(1) {
            let elements: Vec<usize> = map.split_whitespace().map(|n| n.parse()).collect::<Result<Vec<_>, ParseIntError>>()?;
            if elements.len() != 3 {
                return Err(ParseError::LineMalformed(map));
            }
            maps.push([elements[0], elements[1], elements[2]]);
        }
        maps.sort_by(|a, b| a[1].cmp(&b[1]));
        seed_ranges.sort();
        seeds.iter_mut().for_each(|seed| {
            for map in &maps {
                if (map[1]..map[1]+map[2]).contains(seed) {
                    *seed = *seed + map[0] - map[1];
                    break;
                }
            }
        });
        handle_range_mapping(&mut seed_ranges, &maps);
    }

    Ok((*seeds.iter().min().unwrap_or(&0), seed_ranges.iter().map(|sr| sr.0).min().unwrap_or(0)))
}

fn handle_range_mapping(ranges: &mut Vec<(usize, usize)>, maps: &[[usize; 3]]) {
    let mut helper = Vec::new();
    for range in &*ranges {
        // Construct helpers to keep track of which element we are currently looking at
        // and how many are left in this range.
        let mut cursor = range.0;
        let mut remaining = range.1;
        // Look for our cursor in the sorted list of maps. If it is present at index m, we will
        // get Ok(m). Otherwise we'll get Err(n) with n being the next index. In that case, the
        // cursor must be either in map n-1, or unmodified.
        let query = maps.binary_search_by(|map| map[1].cmp(&cursor));
        // Special Case if cursor is below the lowest mapping: Keep elements up to the lowest
        // mapping - 1 unmodified, unless we are done before.
        // This is logically the same as Case C, 
        // but with map_idx=-1, which isn't allowed by the type system.
        if query == Err(0) {
            let next_cursor = maps[0][1];
            if next_cursor >= cursor + remaining {
                helper.push((cursor, remaining));
                continue;
            } else {
                helper.push((cursor, next_cursor-cursor));
                remaining = remaining + cursor - next_cursor;
                cursor = next_cursor;
            }
        }
        let mut map_idx = match query {
            Ok(m) => m,
            Err(0) => 0,
            Err(n) => n-1,
        };
        // We loop as long as we still have remaining elements in our range.
        loop {
            let map = &maps[map_idx];
            // Case A - Map contains Cursor: Push modified elements up to the end of the mapping,
            // unless we are done before.
            if (map[1]..map[1]+map[2]).contains(&cursor) {
                let covered = map[1] + map[2] - cursor;
                if covered >= remaining {
                    helper.push((cursor + map[0] - map[1], remaining));
                    break;
                } else {
                    helper.push((cursor + map[0] - map[1], covered));
                    cursor += covered;
                    remaining -= covered;
                    if map_idx < maps.len()-1 && cursor == maps[map_idx+1][1] {
                        map_idx += 1;
                    }
                }
            // Case B - Cursor is past the last mapping. Just keep everything unmodified.
            } else if map_idx == maps.len()-1 {
                helper.push((cursor, remaining));
                break;
            // Case C - Cursor is between this and the next mapping. Keep everything until the
            // start of the next mapping - 1 unmodified, unless we are done before.
            } else {
                let next_cursor = maps[map_idx+1][1];
                if next_cursor >= cursor + remaining {
                    helper.push((cursor, remaining));
                    break;
                } else {
                    helper.push((cursor, next_cursor-cursor));
                    remaining = remaining + cursor - next_cursor;
                    cursor = next_cursor;
                    map_idx += 1;
                }
            }
        }
    }
    std::mem::swap(ranges, &mut helper);
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
        assert_eq!(run(&sample_input), Ok((35, 46)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((165788812, 1928058)));
    }
}
