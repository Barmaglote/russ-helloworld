fn main() {
    let hello = "Здравствуйте";
    let answer = &hello[0..4]; // Потому что unicode и каждый символ это два байта

    println!("{}", answer)
}
