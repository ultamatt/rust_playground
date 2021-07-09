use std::collections::HashMap;

//Chapter 8, common data types
fn main() {
    do_vectors();
    do_strings();
    do_hashmap();
}

fn do_hashmap(){
    //A hashmap is basically JSON
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn do_strings(){
    //Default way to make a new string
    let mut s = String::new();

    let data = "initial contents";
    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();
    println!("String initial contents are : {}", s2);

    let s3 = String::from("third initial contents");
    println!("String initial contents are : {}", s3);

    //Strings are just wrappers over Vector u8s.... Which explains a lot
    //each char in the string takes two bytes or more.. so using arrays with indexes doesn't work well.

    //Iterate like this preferably
    let mut i = 0;
    for c in "नमस्ते".chars() {
        println!("[{}] {}", i, c);
        i = i + 1;
    }

    i = 0;
    for b in "नमस्ते".bytes() {
        println!("[{}] {}", i, b);
        i = i + 1;
    }
}

fn do_vectors(){
    //A vector is basically a better array
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    for i in &v {
        println!("{}", i);
    }

    //Can declare it like this too, in place
    let mut v2 = vec![1, 2, 3, 4, 5];
    v2.push(6);

    let third: &i32 = &v2[2]; //Reference strictrly, might not be awesome
    println!("The third element is {}, via direct ref", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}, via built in getter", third),
        None => println!("There is no third element."),
    }

    //Yes, vector elements must all be the same type.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // ... but that type can be an enum, which can be many different types
    // this is how to get around that
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
