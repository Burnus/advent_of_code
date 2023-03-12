struct Layer {
    pixels: Vec<Vec<u8>>,
}

impl Layer {
    fn from(bytes: &[u8], width: usize) -> Self {
        Self { 
            pixels: bytes.chunks(width)
                         .map(|c| c.iter()
                                   .map(|b| b-b'0')
                                   .collect())
                         .collect()
        }
    }

    fn count(&self, target: u8) -> usize {
        self.pixels.iter().flatten().filter(|&pix| *pix == target).count()
    }
}

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn from(value: &str, width: usize, height: usize) -> Self {
        Self {
            layers: value.as_bytes().chunks(width*height).map(|c| Layer::from(c, width)).collect(),
            width,
            height,
        }
    }

    fn print(&self) -> String {
        (0..self.height).map(|row| (0..self.width).map(|col| self.layers.iter().find_map(|l| match l.pixels[row][col] {
            0 => Some(' '),
            1 => Some('#'),
            _ => None,
        }).unwrap()).chain(['\n'].into_iter()).collect::<String>())
        .collect()
    }
}

pub fn run(input: &str) -> (usize, String) {
    let image = Image::from(input, 25, 6);
    let layer = image.layers.iter().min_by_key(|l|l.count(0)).unwrap();
    let first = layer.count(1) * layer.count(2);
    let second = image.print();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let img = r#"
#   ##    #### ###    ## 
#   ##    #    #  #    # 
 # # #    ###  #  #    # 
  #  #    #    ###     # 
  #  #    #    #    #  # 
  #  #### #    #     ##  
"#;
        assert_eq!(run(&challenge_input), (1072, img[1..].to_string()));
    }
}
