#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };

    println!("{:#?}", home);
    println!("{:#?}", loopback);
}
