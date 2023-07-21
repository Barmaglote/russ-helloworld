fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s_1 = format!("{s1}-{s2}-{s3}");
    let s_2 = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s_1);
    println!("{}", s_2);
}
