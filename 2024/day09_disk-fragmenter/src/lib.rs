use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseCharError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseCharError(e) => write!(f, "Unable to parse {e} into a digit."),
        }
    }
}

type Id=usize;
type Size=u8;

#[derive(Clone, Debug)]
struct Filesystem {
    // All the FS blocks. `None` for empty blocks, `Some(file_id, file_size)` for files.
    blocks: Vec<Option<(Id, Size)>>,
    // List of the empty blocks in order. Format: (first_block_idx, size)
    free: Vec<(usize, Size)>,
}

impl TryFrom<&str> for Filesystem {
    type Error=ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut file = true;
        let mut id = 0;
        let mut blocks = Vec::new();
        let mut free = Vec::new();
        for c in value.chars() {
            if !c.is_ascii_digit() {
                return Err(Self::Error::ParseCharError(c));
            }
            let size = c as Size - b'0';
            let entry = if file { Some((id, size)) } else { None };
            file = if file {
                id += 1;
                false
            } else {
                if size != 0 {
                    free.push((blocks.len(), size));
                }
                true
            };
            (0..size).for_each(|_| blocks.push(entry));

        }
        Ok(Self { blocks, free, })
    }
}

impl Filesystem {
    fn compress(&mut self) {
        while let Some(empty) = self.blocks.iter().position(|b| b.is_none()) {
            let last = self.blocks.pop().unwrap();
            if last.is_some() {
                self.blocks[empty] = last;
            }
            self.free = Vec::new();
        }
    }

    fn compress_unfragmented(&mut self) {
        for id in (0..=self.blocks.iter().rfind(|file| file.is_some()).unwrap_or(&Some((0, 0))).unwrap().0).rev() {
            let old_idx = self.blocks.iter().position(|file| file.is_some() && file.unwrap().0 == id).unwrap();
            let file_size = self.blocks[old_idx].unwrap().1;
            if let Some(free_entry) = self.free.iter_mut().find(|(_block_id, free_size)| *free_size >= file_size) {
                let new_idx = free_entry.0;
                if new_idx < old_idx {
                    free_entry.0 += file_size as usize;
                    free_entry.1 -= file_size;
                    (0..file_size as usize).for_each(|offset| {
                        self.blocks[old_idx + offset] = None;
                        self.blocks[new_idx + offset] = Some((id, file_size));
                    });
                }
            }
        }

    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate().map(|(idx, file)| idx * file.unwrap_or((0, 0)).0).sum()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut fs1 = Filesystem::try_from(input)?;
    let mut fs2 = fs1.clone();
    fs1.compress();
    fs2.compress_unfragmented();
    let first = fs1.checksum();
    let second = fs2.checksum();
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((1928, 2858)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((6607511583593, 6636608781232)));
    }
}
