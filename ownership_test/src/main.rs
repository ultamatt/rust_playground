
/*
    Each value in Rust has a variable that’s called its owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.
*/
fn main() {

    let immutable_string = "Hello Immutable World!"; //Declared here, immutable
    println!("{}", immutable_string);

    let mut mutable_string = String::from("Hello!"); //Declared with string from static method, mutable
    mutable_string.push_str(" Mutable World!!");
    println!("{}", mutable_string);

    //For variables that are stored in the heap, copying variable locations is a bit complex
    let s1 = String::from("Testing");
    let s2 = s1; //This will effectively move this s1 variable into s2, putting s1 out of scope and thus deallocated

    //println!("{}", s1); //This should error
    println!("{}", s2); //This should be okay

    //you can copy/clone them
    //let s1 = String::from("hello");
    let s3 = s2.clone();

    println!("s2 = {}, s3 = {}", s2, s3);

    //However for STACK data, since we easily konw the size, we copy by default
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    //Enter REFRENCES (Yes, pointers)
    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);

    //Also... You can't change a reference unless you explicitly declare it to be mutable.
    //... Fucking Rust
    let mut s = String::from("hello");
    change(&mut s); //You can only have ONE of these!!
    // WHy? To prevent DATA RACES. Mutable references to the same stuff that might get touched at the same time
    /// Rust just 100% nopes out of that from the start

    let mut slicyboi = String::from("hello world");
    let word = first_word(&slicyboi);
    println!("the first word is: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

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
