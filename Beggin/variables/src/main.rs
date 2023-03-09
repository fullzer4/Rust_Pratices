fn main() {
    let age;
    let name = "gabriel";
    age = 16 + 1;
    println!("Hello, world! From {} and im {}", name, age);
    let number: u32 = 14;
    println!("The number is {}", number);
    let number_f: f32 = 14.5;
    println!("The number is {}", number_f);
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger); 
    let character_1: char = 'S';
    let character_2: char = 'f';
    let smiley_face = '😃';
    let string_1 = "miley ";
    let string_2: &str = "ace";
    
    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
}
