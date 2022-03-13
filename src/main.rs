use text_io::read;

fn main() {
    let some_value = 123;
    println!("Hello, world! {}", some_value);

    println!("Please insert the nth position of the Fib number: ");
    let nth_pos: u64 = read!();

    println!("Result is: {}", fibonacci(nth_pos));
}

fn fibonacci(pos: u64) -> u64 {
    let mut array = [0, 1];
    for _ in 0..pos {
        let temp: u64 = array[0];
        array[0] = array[0] + array[1];
        array[1] = temp;
    }
    return array[0];
}
