fn main() {
    for c in "Зд".chars() {
        println!("{c}");        // Выводит символы
    }

    for b in "Зд".bytes() {
        println!("{b}");        // Выводит байты (4 штуку, по две на каждый символ, потому что юникод)
    }
}
