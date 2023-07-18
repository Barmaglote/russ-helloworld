#[derive(Debug)]
enum Queue {
    Message(u32, u32, u32),
    Work(String)
}

fn main() {
    let que = Queue::Message(5,5,0);

    match que {
        Queue::Message(first, _, _) => {
            println!("Первый элемент Message: {}", first);
        },
        Queue::Work(_) => {
            println!("Первый элемент не является Message.");
        }
    }
}
