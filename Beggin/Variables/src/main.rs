fn main() {
    //let x = 5; // nao pode mudar mas da para adicionar a ela x = x + 1
    //let mut y = 5; // pode mudar
    //y = 6;
    //const SCORE_LIMIT: u32 = 100;
    let mut var: i32 = 1;
    println!("The value of var is {}", var);
    let boolean:bool = true;
    println!("The value of boolean is {}", boolean);
    const SCORE_LIMIT: u32 = 100;
    println!("The value of SCORE_LIMIT is {}", SCORE_LIMIT);
    var = 2;
    println!("Now the value of var is {}", var);
    const STRING: &str = "hello";
    println!("The value of STRING is {}", STRING);
    let array: [u32; 3] = [1,2,3];
    let first_item_a = array[0];
    println!("The value of first_itemA is {}", first_item_a);
    let tuple: (bool, u16, u8) = (true, 2, 3);
    let first_item_t = tuple.0;
    println!("The value of first_itemT is {}", first_item_t);
}
