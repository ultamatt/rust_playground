//  enumeration!
// Here's the types of IP Addresses we might encounter
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), //Putting data right inside this enum
    V6(String),
}

//Enums don't all have to be the same type either
#[derive(Debug)]
enum Message {
    Quit,                    //undefined
    Move { x: i32, y: i32 }, //anonymous struct
    Write(String),           //stringy boi
    Color(i32, i32, i32),    //tuple boi
}

impl Message {
    //Just like structs, you can implement flipping methods
    fn show_me(&self) {
        // method body would be defined here
        println!("Your Message is {:?}", self) //This isn't working because no standard display format
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("LULU!");
            1
        },
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
    //So you COULD make a struct for all this, or just store it right inside the enum
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    //let home = IpAddr::V4(String::from("127.0.0.1")); // Why not just use an enum with different things inside it?

    println!("\n--ENUMERATION--");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("V4 address is {:?}", home);
    println!("V6 address is {:?}", loopback);
    route(home);

    println!("\n--ENUMERATION WITH LOTS OF TYPES--");
    let quit_message = Message::Quit;
    quit_message.show_me();
    let move_message = Message::Move { x: 1, y: 2 };
    move_message.show_me();
    let write_message = Message::Write(String::from("hello"));
    write_message.show_me();
    let color_message = Message::Color(255, 0, 128);
    color_message.show_me();

    println!("\n--OPTION TYPE--");
    // Rust doesn't have NULL!
    // Instead it has Options! This is how 'null' is done in rust
    /*
        Here's what that looks like under the covers

        enum Option<T> {
            None,
            Some(T),
        }

        Basically, your options are 'none' (Nothing!);
        or 'some' (Something!)

        This is pretty cool, because you don't have to deal with null references
    */

    let some_number = Some(5); //Something! It's a thing!
    let some_string = Some("a string"); //Another thing!
    println!("{:?}", some_number);
    println!("{:?}", some_string);

    //When using 'None' we gotta tell Rust what type this thing might be someday
    // so it can allocate memory for it
    let absent_number: Option<i32> = None; //Not a thing!
    println!("{:?}", absent_number);

    println!("\n--MATCHING--");
    println!("Penny {:?}", value_in_cents(Coin::Penny));
    println!("Nickel {:?}", value_in_cents(Coin::Nickel));
    println!("Dime {:?}", value_in_cents(Coin::Dime));
    println!("Quarter {:?}", value_in_cents(Coin::Quarter));

    println!("\n--OPTIONS IN FUNCTIONS--");
    let five = Some(5);
    println!("{:?}", plus_one(five));
    println!("{:?}", plus_one(None));

    //Match is exhaustive. so '_' covers all other cases. aka 'default' from JS
    // using _ will basically drop the value and do nothing
    // in other cases, we can use a variable name to grab the value
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        other => println!("Its {}", other),
        //_ => (),
    }

    println!("\n--IF LET FLOW CONTROL--");
    // So you know how to match eh? How about a smaller way of using Some,
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum matchy is configured to be {}", max),
        _ => (),
    }

    // We could do this instead!
    // it's a destructuring syntax it seems
    if let Some(max) = config_max {
        println!("The maximum let some is configured to be {}", max);
    }


    //This is basically sugar syntax!
}

fn route(da_ip: IpAddr) {
    println!("Routing {:?}", da_ip);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // In rust, a match statement MUST cover all possible cases!
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
