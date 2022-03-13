use text_io::read;

mod math;

fn main() {
    let some_value = 123;
    println!("Hello, world! {}", some_value);

    println!("Please insert the nth position of the Fib number: ");
    let nth_pos: u32 = read!();

    println!("Result is: {}", math::fibonacci(nth_pos));
}
