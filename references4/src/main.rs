fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s // Здесь мы возращаем не ссылку на объект, который удаляется, а сам объект, поскольку мы его перемещаем, то он будет существовать и позже.
}