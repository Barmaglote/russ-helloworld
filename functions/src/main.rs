fn main() {
    println!("Hello, world!");

    let mut label = String::new();
    label = "Label".to_string();

    another_function(5, label.to_string());
}

fn another_function(x: i32, label: String) {
    println!("Another function. {x}, {label}");
}