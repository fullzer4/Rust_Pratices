fn main() {
    let mut nums:Vec<i32> = vec![1,2,3];

    nums.push(4);
    println!("{:#?}", nums);
    nums.pop();
    println!("{:#?}", nums);

    let mut vec:Vec<&str> = Vec::new();

    vec.push("test");
    vec.push("string");
    println!("{:#?}", vec);

    vec.reverse();
    println!("{:#?}", vec);

    let mut _vect:Vec<i32> = Vec::<i32>::with_capacity(2);
    println!("{}", _vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:#?}", v)
}
