fn main() {

    let tuple_e = ('E', 5i32, true);

    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct
    struct Unit;

    


}
