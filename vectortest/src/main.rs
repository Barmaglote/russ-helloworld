fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);

    println!("{:#?}", v);

    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);

    match third {
        Some(third) => println!("Yeah, The third element is {third}"),
        None => println!("There is not third element.")
    }
}
