//CH15 Smart Pointers
// Turns out we've used smart pointers already a bit.
// Strings and 'Box<T>' are instances of it.
// It's a primitive that represents a pointer on the stack that points to the heap

/*

    Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable
      borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside
      the RefCell<T> even when the RefCell<T> is immutable.

*/

// CH15 - 1
// use crate::List::{Cons, Nil};
//
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// CH 15 - 2 Implementing the Deref ability
// Deref allows something to be easily de-referenceable
// use std::ops::Deref;
// struct MyBox<T>(T);
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
//
// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }
//
// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
// }

// CH 15 - 3 Implementing Drop
// Drop specifies whats going to happen when your variable goes out of scope
// struct CustomSmartPointer {
//     data: String,
// }
//
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }
//
// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     std::mem::drop(d);
//     //included in prelude, can also just call as 'drop(d);'
//     println!("CustomSmartPointers created.");
// }

// CH 15 - 4 Reference Counting Types
// Reference Count type, or RC<T> allows us to free memory after all
//  references are accounted for and done being used
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
// use std::rc::Rc;
//
// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

// CH 15 - 5 RefCell Types
// Sometimes you need to do unsafe stuff and have mutability internally
// but everything is immutable by default, so we're going to do some immutable API wrapping
//  and just promise the rust compiler we know what we're doing
//fn main() {
    // let x = 5;
    // let y = &mut x;
    // I honestly don't understand this one.
//}

// CH 15 - 6 On Memory Leaking
// Having references that never clear up will allocate memory forever and never give it back

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());
}
