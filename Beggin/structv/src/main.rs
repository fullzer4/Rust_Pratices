struct Car{
    make: String,
    model: String,
    year: u32,
}

struct Point2d(u32, u32);

fn main() {

    let car1 = Car {
        make: String::from("Ford"),
        model: String::from("Mustang"),
        year: 1967,
    };

    println!("car value make: {}", car1.make);
    println!("car value model: {}", car1.model);
    println!("car value year: {}", car1.year);

    let origin = Point2d(100, 200);

    println!("origin contains {:?} and {:?} values", origin.0, origin.1)
}