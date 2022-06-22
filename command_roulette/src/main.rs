
struct Command {
    number: i32,
    to_do: Box<dyn Fn() -> ()>,
}

fn main() {
    println!("Hello, world!");
    let command_one = Command {
        number: 1,
        to_do: Box::new(|| {
            println!("We did command one");
        })
    };

    let command_two = Command {
        number: 2,
        to_do: Box::new(|| {
            println!("We did command two");
        })
    };

    let commands = vec![command_one, command_two];
    for x in commands.iter() {
       (x.to_do)();
   }
}
