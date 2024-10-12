// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // struktura jednostkowa
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // struktura krotkowa
struct ChangeColorMessage(i32, i32, i32); // struktura krotkowa

impl Message {
    fn call(&self) {
        //
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    //
    // route(IpAddrKind::V6);
    // route(IpAddrKind::V4);
    //
    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6("::1");

    let m = Message::Write(String::from("Witaj"));
    m.call();

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Maksymalna wartość: {}", max),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("Maksymalna wartość: {}", max);
    }
}

// fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("Cwiartka ze stanu: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}