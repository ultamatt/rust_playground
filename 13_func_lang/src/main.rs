//Chapter 13, closures and other shit from other languages
use std::thread;
use std::time::Duration;

extern crate rand;

use rand::thread_rng;
use rand::Rng;

struct Cacher<T> where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T> where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => {
                //println!("We already have a value in this Cacher, using that.");
                v
            },
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_result = Cacher::new(|num| {
        //println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num+1
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

//A closure is a function you can schlep around as if it were a variable
fn main() {
    let mut rng = thread_rng();

    for n in 1..2 {
        let simulated_user_specified_value : i32 = rng.gen_range(0, 100);
        let simulated_random_number : i32 = rng.gen_range(0, 10);
        println!("-------> ");
        println!("\tN is {}, Intensity is {}, Random is {}", n, simulated_user_specified_value, simulated_random_number);
        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    println!("--> Vector Stuff <--");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2 = v1.to_vec();
    let v3 = v1.to_vec();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let daSum : i32 = v2.iter().sum();
    println!("Sum is {}", daSum);

    let daPlusOne : Vec<i32> = v3.iter().map(|x| x + 1).collect();
    println!("Increm {:?}", daPlusOne);


}
