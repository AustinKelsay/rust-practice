enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);

fn route(ip_type: IpAddrKind) {
    
    let localhost = IpAddr {
        kind: ip_type,
        address: String::from("127.0.0.1")
}
