/*
    A file to explain basics of Rust
    follows along with chapter 3 of the rust book
*/

fn main() {

    do_variables();
    do_data_types();
    do_functions();
    do_flow_control();
}

fn do_variables(){
    ////////////////
    //3.1 VARIABLES!
    ////////////////

    //VARIABLES and MUTABILITY
    //For a variable to be changed, is must MUTABLE
    // in rust, variables are IMMUTABLE by default
    println!("\n--VARIABLE MUTABILTY--");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // CONSTANTS
    println!("\n--CONSTANTS--");
    const MAX_POINTS: u32 = 100_000; //Constants are always immutable, must have types
    // Rust allows understores '_' inside numbers to improve readability
    println!("The MAX_POINTS constant is: {}", MAX_POINTS);

    // SHADOWING
    println!("\n--SHADOWING--");
    let x = 5;
    // by redeclaring x, we're actually making a shadow copy.
    let x = x + 1;
    // this shadow copy is still immutable
    let x = x * 2;
    // this is mainly useful to operate on a variable, but still make it IMMUTABLE
    // similar to state manipulation in redux
    println!("The value of x is: {}", x);
    // it's also good for variable type reassignment
    let spaces = "   "; //a string
    let spaces = spaces.len(); //a number
    // let mut spaces = "   ";  <-- This wont work, because non-shadowing won't let us
    // spaces = spaces.len();        change the type assigned
    println!("The number of spaces are: {}", spaces);
}

fn do_data_types(){

    ////////////////
    //3.2 DATA TYPES!
    ////////////////

    // Data types in rust are either SCALAR or COMPOUND
    // SCALAR types represent ONE value
    /// Examples: integers, floating-point numbers, Booleans, and characters
    // COMPOUND types group multiple values into one type.
    /// Examples: tuples and arrays.

    ////////////////////
    // SCALAR TYPES !!
    //Integers, it's basic whole numbers, positive or negative
    println!("\n--INTEGERS--");
    let in8: i8 = -128;
    println!("Integer 8 bytes minimum val is: {}", in8);
    let un128: u128 = u128::MAX;
    println!("Unsigned Integer 128 bytes max val is: {}", un128);

    //Floats! 32's have 1 precision, 64's have 2 precision
    println!("\n--FLOATS--");
    let fl32: f32 = f32::MIN;
    println!("Float 32 bytes minimum val is: {}", fl32);
    let fl64: f64 = f64::MAX;
    println!("Float 64 bytes maximum val is: {}", fl64);

    //Booleans!
    println!("\n--BOOLEANS--");
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("True is: {} False is: {}", t, f);

    //Characters
    // Literal characters are specified with SINGLE quotes
    // Strings are specified with DOUBLE quotes
    // Each char is 4 bytes and uses UNICODE SCALAR values,
    println!("\n--CHARACTERS--");
    let cchar = 'z';
    let zchar = 'ℤ';
    let heart_eyed_cat_char = '😻';
    let jchar = '水';
    println!("char z {} char big Z {} emoji cat {} japanese {}", cchar, zchar, heart_eyed_cat_char, jchar);

    ////////////////////
    // COMPOUND TYPES!!
    // Tuples are groupings of scalar variables from above
    println!("\n--TUPLES--");
    let tup: (i32, f64, bool, char) = (500, 6.4, true, '赤');
    // the whole variable now has all 4 elements. You have to use destructuring (below) to grab values out
    let (number, float, boolean, character) = tup;
    println!("number {} float {} boolean {} japanese {}", number, float, boolean, character);
    // You can also use dot notation to grab individual elements
    println!("{} is my favorite number", tup.0);

    //Arrays
    // All the elements in the array MUST be the SAME TYPE
    println!("\n--ARRAYS--");
    let daArray = [1, 2, 3, 4, 5];
    // You can also define the type and number of elements up front too
    let typeDefinedArray: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} is the last element in this array", typeDefinedArray[4]);
}

fn do_functions(){

    ////////////////
    //3.3 FUNCTIONS!
    ////////////////
    println!("\n--FUNCTIONS--");
    another_function(5, 6);
}

//Rust uses SNAKE CASE for function and variable names
// When passing variables, you must specify type
fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    //STATEMENTS vs EXPRESSIONS
    //Statements are instructions that perform some action and do not return a value.
    //Expressions evaluate to a resulting value, they return something.
    // In c, you can do this "x = y = 6" because assignment returns the value you assigned.
    /// NOT SO in RUST
    let new_x = 5;

    let new_y = {
        let new_x = 3;
        new_x + 1
        //NOTE NO SEMICOLON here. this means it's an EXPRESSION
        // If we added a semicolon, it'd be a statement and we'd not get anything back
    };

    println!("The value of new_y is: {}", new_y);
    println!("This should say the number five: {}", five());
    println!("This should say the number ten: {}", ten());
}

//This function returns a type i32, which is the number 5 as an expression
fn five() -> i32 {
    5
}

//Explicit return is okay too
fn ten() -> i32 {
    return 10;
}

fn do_flow_control(){
    ////////////////
    //3.5 FLOW CONTROL
    ////////////////
    println!("\n--FLOW CONTROL--");
    let condition = true;
    let mut number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    //IF and ELSE and ELSE IF
    //Conditions in rust MUST be a boolean. No coercion here
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //LOOPS
    let mut counter = 0;

    //Loops can return an expression to use
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //While Loop
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //FOR x in y loops
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //Also making a quick array/collection and reversing it
    for number in (1..=3).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
