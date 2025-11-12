use std::fs;

fn get_contents(path: &'static str) -> String {
    let contents = fs::read_to_string(path).expect(&format!("Error reading {}", path));

    contents
}

fn main() {
    // let mut inputs_1: Vec<u32> = Vec::new();
    // let mut inputs_2: Vec<u32> = Vec::new();

    let path = "src/inputs.txt";
    let contents = get_contents(path);

    for c in contents.chars() {
        let mut processing = true;

        while processing {
            if c.is_numeric() {
            } else if c == ' ' {
                processing = !processing;
            }
        }
    }
}
