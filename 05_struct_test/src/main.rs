// Baby's first struct
#[derive(Debug)]
struct User {
    username: String, // Why not use a reference to a string here? &str?
    // WAHELP it's because we want our struct to own all the data inside it, so it's really
    //  clear and easy to understand when the lifetime is up and rust can garbage collect
    //  if its got references to stuff in it that OTHER pieces of the code own,
    //  we have to explicitly tie the lifetime into something else using a NAMED LIFETIME PARAMETER
    //  this is covered in chapter 10 and TOO HARD FOR BABY RIGHT NOW
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Structs which are just tuples!
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// even though they're the same structure, rust won't allow us to
//  assume one is the 'same' as the other

// A unit-like struct
struct AlwaysEqual;

// A rectangle with debug! So that Rust can understand how to print out our Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Functions implemented on the Rectangle
// Separated into two, to show we can
// Items here are for instances
impl Rectangle {
    //first argument is always self
    fn width(&self) -> u32 {
        self.width
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Borrowing here so we don't have ownership issues
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Items here are constructor/static
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {

    //Entire instance is mutable, rust won't let you do individual fields here
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("someone2@example.com"),
        String::from("someusername2"),
    );

    let user3 = User {
        //email: String::from("another@example.com"),  // <- Borrowing email here when commented out
        username: String::from("anotherusername567"),
        ..user2 //This is a struct update syntax
    };


    println!("\n--STRUCTS AND STRUCT UPDATE SYNTAX--");
    // dbg (debug) macro is super cool. Prints line number and debug info
    //  all this comes over stderr though, so watch out!
    dbg!(&user1);

    //println!("user2 should error {}", user2.email); // <-- Doesn't exist as user2 anymore. Data got moved
    println!("user3 email {}", user3.email);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(40);

    /*
        Where’s the -> Operator?

        In C and C++, two different operators are used for calling methods:
        you use . if you’re calling a method on the object directly
        and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first.
        In other words, if object is a pointer, object->something() is similar to (*object).something().

        Rust doesn’t have an equivalent to the -> operator;
        instead, Rust has a feature called automatic referencing and dereferencing.
        Calling methods is one of the few places in Rust that has this behavior.

        Rust automatically adds in &, &mut, or * so object matches the signature of the method when you call it

        In other words, the following are the same:
        p1.distance(&p2);
        (&p1).distance(&p2);

        This automatic referencing behavior works because methods have a clear receiver—the type of self.
        Given the receiver and name of a method, Rust can figure out definitively whether the method is
        reading (&self), mutating (&mut self), or consuming (self).
    */

    println!("\n--STRUCTS AND STRUCT METHODS--");
    println!("rect1 has width {}", rect1.width());
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
