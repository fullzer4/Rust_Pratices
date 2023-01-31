fn main() {

    if 1 == 2 {
        println!("math is broken")
    } else {
        println!("fine")
    }

    let formal: bool = true;
    let _greeting: () = if formal {
        println!("Good evening")
    } else {
        println!("Hey, friend")
    };

    let number:i32 = 6;

    if number % 4 == 0 {
        println!("divisible by 4")
    } else if number % 3 == 0 {
        println!("divisible by 3")
    } else{
        println!("not divisible by 3 and 4")
    }

    let boolean: bool = true;

    let _binary: i32 = match boolean {
        false => 0,
        true => 1
    };
}
