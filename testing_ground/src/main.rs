/*
    A file to explain basics of Rust
    follows along with chapter 3 of the rust book
*/

fn main() {
    //3.1 VARIABLES!

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

    //3.2 DATA TYPES!
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
    println!("{} is my favorite number", tup.0)

    //Arrays
    // All the elements in the array MUST be the SAME TYPE
    println!("\n--ARRAYS--")
    let daArray = [1, 2, 3, 4, 5];
    // You can also define the type and number of elements up front too
    let typeDefinedArray: [i32; 5] = [1, 2, 3, 4, 5];
}
