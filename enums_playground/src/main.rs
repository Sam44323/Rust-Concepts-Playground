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

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// example of implementing a method on an enum
impl Message {
    fn call(&self, message: String) {
        println!("{}: {:?}", message, &self);
    }
}

fn multi_embed_enum_example() {
    let quit = Message::Quit;
    let move_data = Message::Move { x: 10, y: 15 };
    let write = Message::Write(String::from("hello"));
    let chang_color = Message::ChangeColor(255, 0, 255);

    quit.call(String::from("Quit"));
    move_data.call(String::from("Move"));
    write.call(String::from("Write"));
    chang_color.call(String::from("Change-Color"));
}

fn main() {
    basic_enun_init();
    enum_embedded_example();
    multi_embed_enum_example();
}
