use text_io::read;

mod math;

fn main() {
    let some_value = 123;
    println!("Hello, world! {}", some_value);

    println!("Please insert the nth position of the Fibonacci/Lucas number: ");
    let mut nth_pos: u32 = read!();
    nth_pos -= 1;

    println!(
        "Result is: [Fib={}, Lucas={}]",
        math::fibonacci(nth_pos),
        math::lucas(nth_pos)
    );
}
