struct Ingredient {
    capacity: isize,
    durability: isize,
    flavour: isize,
    texture: isize,
    calories: usize,
}

impl Ingredient {
    fn parse(line: &str) -> Self {
        let components: Vec<_> = line.split(' ').collect();
        assert_eq!(components.len(), 11);
        Self {
            capacity: strip_last_char(components[2]).parse().unwrap(),
            durability: strip_last_char(components[4]).parse().unwrap(),
            flavour: strip_last_char(components[6]).parse().unwrap(),
            texture: strip_last_char(components[8]).parse().unwrap(),
            calories: components[10].parse().unwrap(),
        }
    }
}

fn strip_last_char(string: &str) -> &str {
    &string[..string.len()-1]
}

pub fn run(input: &str) -> (usize, usize) {
    let ingredients: Vec<_> = input.lines().map(Ingredient::parse).collect();

    let first = try_combinations(&ingredients, None);
    let second = try_combinations(&ingredients, Some(500));
    (first, second)
}

fn try_combinations(ingredients: &Vec<Ingredient>, cal_requirement: Option<usize>) -> usize {
    let ingredient_count = ingredients.len();
    let amounts = vec![0; ingredient_count];

    stars_and_bars(100, ingredient_count, &amounts, ingredients, cal_requirement)

}

fn stars_and_bars(stars: u8, bars: usize, amounts: &[u8], ingredients: &[Ingredient], cal_requirement: Option<usize>) -> usize {
    match bars {
        0 => get_score(amounts, ingredients, cal_requirement),
        1 => {
                let mut new_amounts = amounts.to_vec();
                new_amounts[0] = stars;
                stars_and_bars(0, 0, &new_amounts, ingredients, cal_requirement)
            },
        _ => {
                (0..=stars).map(|i| {
                    let mut new_amounts = amounts.to_vec();
                    new_amounts[bars-1] = i;
                    stars_and_bars(stars-i, bars-1, &new_amounts, ingredients, cal_requirement)
                }).max().unwrap_or(0)
            },
    }
}

fn get_score(amounts: &[u8], ingredients: &[Ingredient], cal_requirement: Option<usize>) -> usize {
    let calories = amounts.iter().enumerate().map(|(idx, &a)| a as usize * ingredients.get(idx).unwrap().calories).sum::<usize>();
    if cal_requirement.is_some() && Some(calories) != cal_requirement {
        return 0;
    }

    let capacity = amounts.iter().enumerate().map(|(idx, &a)| a as isize * ingredients.get(idx).unwrap().capacity).sum::<isize>().max(0);
    let durability = amounts.iter().enumerate().map(|(idx, &a)| a as isize * ingredients.get(idx).unwrap().durability).sum::<isize>().max(0);
    let flavour = amounts.iter().enumerate().map(|(idx, &a)| a as isize * ingredients.get(idx).unwrap().flavour).sum::<isize>().max(0);
    let texture = amounts.iter().enumerate().map(|(idx, &a)| a as isize * ingredients.get(idx).unwrap().texture).sum::<isize>().max(0);
    
    (capacity * durability * texture * flavour) as usize
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
        assert_eq!(run(&sample_input), (62842880, 57600000));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (222870, 117936));
    }
}
