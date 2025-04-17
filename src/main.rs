use a_bit_rusty::add;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough numbers to add! Provide at least two.");
        return;
    }

    let a = args[1].parse::<u64>().unwrap_or(0);
    let b = args[2].parse::<u64>().unwrap_or(0);
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
}
