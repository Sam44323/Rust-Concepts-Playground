// example of creating an enum
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

fn basic_enun_init() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}", home.kind);
    println!("home: {:?}", loopback.kind);
}

#[derive(Debug)]
enum IpAddrKindEmbed {
    V4(String),
    V6(String),
}

fn enum_embedded_example() {
    let home = IpAddrKindEmbed::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindEmbed::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("home: {:?}", loopback);
}

fn main() {
    basic_enun_init();
    enum_embedded_example();
}
