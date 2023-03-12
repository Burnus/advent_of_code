use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn main() {
    let contents = read_file("sample_input");
    //let contents = read_file("input");
}
