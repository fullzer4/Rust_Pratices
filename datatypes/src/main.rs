fn main() {
    let x: i8 = 10;
    println!("{}", x)

    let y: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}",byte);

    let xf = 2.00;
    let yf: f32 = 1.0;

    let t = true;
    let f: bool = false;

    let c = 'c';

    println!("{}",c)

    let a = 10;
    let b = 5;

    let remeder  = a % b;
    println!("{}", remeder)
}
