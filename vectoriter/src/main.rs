fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }


    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("{i}");
    }
}
