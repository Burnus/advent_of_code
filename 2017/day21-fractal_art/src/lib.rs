use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Image {
    pixels: Vec<Vec<bool>>,
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        Self { 
            pixels: value.split('/')
                .map(|line| line.chars().map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Unrecognized Token: {c}"),
                    }).collect()
                ).collect(),
        }
    }
}

impl Image {
    fn split(&self, size: usize) -> Vec<Self> {
        (0..self.pixels.len()/size).flat_map(|y_iteration| 
            (0..self.pixels.len()/size).map(|x_iteration| 
                    Self { pixels: (0..size).map(|y| 
                            (0..size).map(|x| self.pixels[y_iteration*size + y][x_iteration * size + x])
                            .collect::<Vec<bool>>()
                        ).collect::<Vec<Vec<bool>>>()
                    }).collect::<Vec<Self>>()
            ).collect()
    }

    fn mirrors_and_rotations(&self) -> Vec<Self> {
        let len = &self.pixels.len();
        let mut res = Vec::from([self.clone()]);
        // mirrors
        res.push(Self { pixels: (0..*len).map(|y| self.pixels[*len-y-1].clone()).collect(), });
        res.push(Self { pixels: (0..*len).map(|y| (0..*len).map(|x|
                                              self.pixels[y][*len-x-1]).collect()).collect(), });

        // rotations
        res.push(Self { pixels: (0..*len).map(|y| (0..*len).map(|x|
                                              self.pixels[x][len-y-1]).collect()).collect(), });
        res.push(Self { pixels: (0..*len).map(|y| (0..*len).map(|x|
                                              self.pixels[*len-y-1][*len-x-1]).collect()).collect(), });
        res.push(Self { pixels: (0..*len).map(|y| (0..*len).map(|x|
                                              self.pixels[len-x-1][y]).collect()).collect(), });

        // both
        res.push(Self { pixels: (0..*len).map(|y| (0..*len).map(|x|
                                              self.pixels[*len-x-1][*len-y-1]).collect()).collect(), });
        res.push(Self { pixels: (0..*len).map(|y| (0..*len).map(|x|
                                              self.pixels[x][y]).collect()).collect(), });


        res
    }

    fn compose(parts: &[Self], blocks: usize) -> Self {
        let block_size = parts[0].pixels.len();
        Self { pixels: (0..blocks).flat_map(|block_row|
                        (0..block_size).map(|row_in_block|
                            parts[block_row*blocks..(block_row+1)*blocks].iter()
                                .flat_map(|part| part.pixels[row_in_block].to_vec())
                            .collect()
                        ).collect::<Vec<Vec<bool>>>()
                    ).collect(),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Substitution {
    from: Image,
    to: Image,
}

impl From<&str> for Substitution {
    fn from(value: &str) -> Self {
        let (l, r) = value.split_once(" => ").unwrap();
        Self{
            from: l.into(),
            to: r.into(),
        }
    }
}

pub fn run(input: &str, iterations: usize) -> usize {
    let mut grid = Image::from(".#./..#/###");
    let substitutions: Vec<_> = input.lines().map(Substitution::from).collect();
    let mut mem: HashMap<Image, Image> = HashMap::new();
    for _ in 0..iterations {
        let size = grid.pixels.len();
        let block_size = 2 + size % 2;
        grid = Image::compose(&grid.split(block_size)
                                    .iter()
                                    .map(|sub| {
                                        if let Some(res) = mem.get(sub) {
                                            res.clone()
                                        } else {
                                            let variants = sub.mirrors_and_rotations();
                                            let res = substitute(&variants, &substitutions);
                                            for v in variants {
                                                mem.insert(v, res.clone());
                                            }
                                            res
                                        }
                                    }).collect::<Vec<Image>>(),
                                size/block_size);
    }
    grid.pixels.iter().flatten().filter(|pixel| **pixel).count()
}

fn substitute(images: &[Image], substitutions: &[Substitution]) -> Image {
    let from_patterns: Vec<_> = substitutions.iter().map(|s| s.from.clone()).collect();
    let variant = images.iter().find(|i| from_patterns.contains(i)).unwrap_or_else(|| panic!("No Substitution found for {:?}", &images[0]));
    substitutions.iter().find(|s| s.from == *variant).map(|s| s.to.clone()).unwrap()
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
        assert_eq!(run(&sample_input, 2), 12);
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 5), 179);
        assert_eq!(run(&challenge_input, 18), 2766750);
    }
}
