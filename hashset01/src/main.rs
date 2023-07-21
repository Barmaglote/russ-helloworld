use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{:#?}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Green"), 25);
    println!("{:#?}", scores);

    scores.insert(String::from("Green"), 55); // Overwriting value
    println!("{:#?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50); // Add value
    scores.entry(String::from("Green")).or_insert(50); // Overwrite value, only if it is not set
    println!("{:#?}", scores);


    // Can be used for aggrigating data
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}





