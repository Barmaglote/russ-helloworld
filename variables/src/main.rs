use std::io;

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The THREE_HOURS_IN_SECONDS of x is {THREE_HOURS_IN_SECONDS}");

    let tup = (500, 300, 700);
    let x = tup.0 + tup.1;

    println!("X from tup {x}");

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("{first}, {second}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
