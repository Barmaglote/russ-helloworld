fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}",s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let a1 = String::from("Hello, ");
    let a2 = String::from("world!");
    let a3 = a1 + &a2; // note s1 has been moved here and can no longer be used, can be fixed with a1.clone()

    println!("{}", a3);

    let hello = "Здравствуйте";
    let answer = &hello[0..4];

    println!("{}", answer);
}
