use text_io::read;

mod math;

fn main() {
    let some_value = 123;
    println!("Hello, world! {}", some_value);

    println!("Please insert the nth position of the Fibonacci/Lucas number: ");
    let mut nth_position: u64 = read!();
    nth_position -= 1;

    println!(
        "Result is: [Fib={}, Lucas={}]",
        math::fibonacci(nth_position),
        math::lucas(nth_position)
    );
}
