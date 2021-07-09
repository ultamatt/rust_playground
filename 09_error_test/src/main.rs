/*
    Working on handling errors!
*/
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    //panic!("crash and burn");
    #![allow(unused)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    do_fileopen();
    read_username_from_file();
    read_username_from_file_2();
}

fn do_fileopen(){
    //let f = File::open("hello.txt");

    //Can do matching on the result here
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Better error handling
    // This one has match expressions to handle stuff
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    //Unwrap just prints a stack trace if you get an error automagically
    //let f = File::open("hello.txt").unwrap();
    //Expect allows us to choose the error message, very similar to unwrap
    //let f = File::open("hello.txt").expect("This is my custom error!");

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
