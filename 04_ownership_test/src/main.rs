/*
    Each value in Rust has a variable that’s called its owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    println!("\n--STRING MUTABILTY--");

    //Declared here, immutable
    let immutable_string = "Hello Immutable World!";
    println!("{}", immutable_string);

    //Declared with string from static method, mutable
    let mut mutable_string = String::from("Hello!");
    mutable_string.push_str(" Mutable World!!");
    println!("{}", mutable_string);

    //For variables that are stored in the heap, copying variable locations is a bit complex
    //This will effectively move this s1 variable into s2, putting s1 out of scope and thus deallocated
    let s1 = String::from("Testing");
    let s2 = s1;

    //println!("{}", s1); //This will error
    println!("{}", s2); //This will be okay

    //you can copy/clone them
    //let s1 = String::from("hello");
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    //However for STACK data, since we easily konw the size, we copy by default
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    println!("\n--FUNCTION OWNERSHIP--");
    // Moving by value
    {
        // gives_ownership moves its return value into s1
        let s1 = gives_ownership();
        println!("S1 is {}", s1);

        // s2 comes into scope
        let s2 = String::from("hello");
        println!("S2 is {}", s2);

        // s2 is moved into takes_and_gives_back, which also moves its return value into s3
        let s3 = takes_and_gives_back(s2);
        println!("S3 is {}", s3);
        //println!("S2 is {}", s2); //This errors, because s2 has been moved!

    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    println!("\n--REFERENCES--");
    //Enter REFRENCES (Yes, pointers)
    // A reference is denoted by ampersand '&'
    // A de-reference is denoted by asterisk '*'
    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);

    //Also... You can't change a reference unless you explicitly declare it to be mutable.
    let mut s5 = String::from("hello");
    change(&mut s5); //You can only have ONE of these!!
    // WHY? To prevent DATA RACES. Mutable references to the same stuff that might get touched at the same time
    // Rust just 100% nopes out of that from the start

    //This works because the immutable references are only reading, AND we basically stop using them after we print them
    let mut s6 = String::from("hello");
    let r1 = &s6; // no problem
    let r2 = &s6; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s6; // no problem
    println!("{}", r3);
    //The ability of the compiler to tell that a reference is no longer being used
    //  at a point before the end of the scope is called Non-Lexical Lifetimes

    // SLICES
    println!("\n--SLICES--");
    let slicyboi = String::from("hello world");
    let word = first_word(&slicyboi);
    println!("the first word is: {}", word);
}

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    // some_string is returned and moves out to the calling function
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    // a_string is returned and moves out to the calling function
    a_string
}

// Function takes a REFERENCE to a string and returns the size of the string
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function takes a reference to a mutable string and returns that string plus the phrase ', world'
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// We can grab slices with the array and .. notation [0..1] etc
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //something followed by a space
            return &s[0..i];
        }
    }

    &s[..]
}
