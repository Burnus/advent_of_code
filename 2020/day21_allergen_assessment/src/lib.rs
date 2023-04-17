use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, String), ParseError> {
    let (ingredients_map, ingredients_occurrences) = parse_food(input)?;
    let mapping = sieve(&ingredients_map, &ingredients_occurrences);
    let first = ingredients_occurrences.iter().filter(|(name, _occurrence_count)| !mapping.iter().any(|(_allergen, ingredient)| ingredient == name)).map(|(_name, occurrence_count)| occurrence_count).sum();
    let second = mapping.into_iter().map(|(_allergen, ingredient)| ingredient).collect::<Vec<_>>().join(",");
    Ok((first, second))
}

fn sieve(mapping: &[(String, Vec<Vec<usize>>)], ingredients_occurrences: &[(String, usize)]) -> Vec<(String, String)> {
    let mut res = Vec::new();
    let mut open_set = mapping.to_vec();

    while !open_set.is_empty() {
        for idx in (0..open_set.len()).rev() {
            let (allergen, ingredients) = open_set[idx].clone();
            let common_ingredients = get_union(&ingredients);
            if common_ingredients.len() == 1 {
                res.push((allergen, ingredients_occurrences[common_ingredients[0]].0.clone()));
                open_set.remove(idx);
                open_set.iter_mut().for_each(|(_allergen, ingredients_lists)| {
                    ingredients_lists.iter_mut().for_each(|ingredients| {
                        (0..ingredients.len()).rev().for_each(|idx| {
                            if ingredients[idx] == common_ingredients[0] {
                                ingredients.remove(idx);
                            }
                        });
                    });
                });
            }
        }
    }
    res.sort();
    res
}

fn get_union(lists: &[Vec<usize>]) -> Vec<usize> {
    let mut res = lists[0].to_vec();
    lists.iter().skip(1).for_each(|list| {
        for idx in (0..res.len()).rev() {
            let elem = res[idx];
            if !list.contains(&elem) {
                res.remove(idx);
            }
        }
    });
    res
}

fn parse_food(list: &str) -> Result<(Vec<(String, Vec<Vec<usize>>)>, Vec<(String, usize)>), ParseError> {
    let mut ingredients = Vec::new();
    let mut allergens = Vec::new();

    let mut map = Vec::new();
    let mut occurrences: Vec<(String, usize)> = Vec::new();

    for line in list.lines() {
        if let Some((ings, alls)) = line.split_once(" (contains ") {
            let ings: Vec<_> = ings.split_whitespace().map(|ing| if let Some(idx) = ingredients.iter().position(|i| i == &ing) { occurrences[idx].1 += 1; idx } else { occurrences.push((ing.to_string(), 1)); ingredients.push(ing); ingredients.len()-1 }).collect();
            let alls: Vec<_> = alls.split(&[' ', ',', ')']).filter(|comp| !comp.is_empty()).map(|all| if let Some(idx) = allergens.iter().position(|a| a == &all) { idx } else { map.push((all.to_string(), Vec::new())); allergens.push(all); allergens.len()-1}).collect();

            for allergen_idx in alls {
                map[allergen_idx].1.push(ings.clone());
            }
        } else {
            return Err(ParseError::LineMalformed(line.to_string()));
        }
    }
    Ok((map, occurrences))
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
        assert_eq!(run(&sample_input), Ok((5, "mxmxvkd,sqjhc,fvjkl".to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2423, "jzzjz,bxkrd,pllzxb,gjddl,xfqnss,dzkb,vspv,dxvsp".to_string())));
    }
}
