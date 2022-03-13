use std::str;
use text_io::read;

fn main() {
    let some_value = 123;
    println!("Hello, world! {}", some_value);

    println!("Please insert the nth position of the Fib number: ");
    let nth_pos: u64 = read!();

    println!("Result is: {}", fibonacci(nth_pos));
}

fn fibonacci(pos: u64) -> u64 {
    let mut vector = vec![0, 1];
    for _ in 0..pos {
        let temp: u64 = vector[1];
        vector[1] = vector[0] + vector[1];
        vector[0] = temp;
    }
    return vector[0];
}
