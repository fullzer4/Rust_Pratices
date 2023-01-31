fn main() {
    println!("{}", exclaim("works"));
    println!("{}", last_char(String::from("Hello")));
}

fn exclaim(input: &str) -> String {
    let mut output = input.to_uppercase();
    output.push('!');
    output
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return '⛔';
    }
    string.chars().next_back().unwrap()
}