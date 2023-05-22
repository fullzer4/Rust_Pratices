fn main() {
    let x: i8 = 10;
    println!("{}", x);

    let _y: u8 = 10;

    let decimal: i32 = 02_55;
    let hex: i32 = 0xff;
    let octal: i32 = 0o377;
    let binary: i32 = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte: u8 = b'A';
    println!("{}",byte);

    let _xf: f64 = 2.00;
    let _yf: f32 = 1.0;

    let _t: bool = true;
    let _f: bool = false;

    let c: char = 'c';

    println!("{}",c);

    let a: i32 = 10;
    let b: i32 = 5;

    let remeder: i32  = a % b;
    println!("{}", remeder)
}
