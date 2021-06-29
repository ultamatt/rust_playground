//enumeration!
enum IpAddr {
    V4(String), //Putting data right inside this enum
    V6(String),
}

//Enums don't all have to be the same type either
enum Message {
    Quit, //undefined
    Move { x: i32, y: i32 }, //anonymous struct
    Write(String), //stringy boi
    ChangeColor(i32, i32, i32), //tuple boi
}

impl Message { //Just like structs, you can implement flipping methods
    fn show_me(&self) {
        // method body would be defined here
        //println!("{}", self) //This isn't working because no standard display format
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}



// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    //
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    //
    //
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));


    let m = Message::Write(String::from("hello"));
    m.show_me();

    //Options! This is how 'null' is done in rust
    let some_number = Some(5); //Something! It's a thing!
    let some_string = Some("a string"); //Another thing!

    //When using 'None' we gotta tell Rust what type this thing might be someday
    // so it can allocate memory for it
    let absent_number: Option<i32> = None; //Not a thing!

    //Match is exhaustive. so '_' covers all other cases. aka 'default'
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn route(ip_kind: IpAddr) {}
