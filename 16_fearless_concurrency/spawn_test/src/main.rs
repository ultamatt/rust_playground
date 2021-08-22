// Rust uses 1:1 threads instead of M:N threads or 'green' threads
// it means we're basically on the metal and not abstracted off it
//  for better and for worse
use std::thread;
use std::time::Duration;

fn main() {
    // // This returns a join handle, which you can use to join the main thread
    // //  which means main thread will wait for the thread to finish
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // // Calling this here will run the main thread first and the other stuff second?
    // handle.join().unwrap();
    //
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    //
    // // Calling this after will interleave the two threads
    // //handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
       println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
