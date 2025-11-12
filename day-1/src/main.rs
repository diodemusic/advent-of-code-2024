use std::fs;

fn get_contents(path: &'static str) -> String {
    let contents = fs::read_to_string(path).expect(&format!("Error reading {}", path));

    contents
}

fn main() {
    let mut inputs_1: Vec<u32> = Vec::new();
    let mut inputs_2: Vec<u32> = Vec::new();

    let path = "src/inputs.txt";
    let contents = get_contents(path);

    let mut assign_to_inputs_1 = true;

    for c in contents.chars() {
        if c.is_numeric() {
            if assign_to_inputs_1 {
                inputs_1.push(c.to_digit(10).expect("Error parsing"));
            } else {
                inputs_2.push(c.to_digit(10).expect("Error parsing"));
            }
        } else if c == ' ' {
            assign_to_inputs_1 = !assign_to_inputs_1;
        }
    }
}
