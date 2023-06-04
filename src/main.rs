use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Hello, world!");
    } else {
        println!("Hello, {}!", args[1]);
    }
}
