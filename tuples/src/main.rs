fn main() {
    let tup = (500, "hi", true);

    println!("{}", tup.1);
    
    let (x, y, z) = tup;

    println!("{}", x);

    println!("{}", y);

    println!("{}", z);
}
