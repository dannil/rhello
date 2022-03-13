fn fib_sequence(first: u32, second: u32, pos: u32) -> u32 {
    let mut vector = vec![first, second];
    for _ in 0..pos {
        let temp: u32 = vector[1];
        vector[1] = vector[0] + vector[1];
        vector[0] = temp;
    }
    return vector[0];
}

pub fn lucas(pos: u32) -> u32 {
    return fib_sequence(2, 1, pos);
}

pub fn fibonacci(pos: u32) -> u32 {
    return fib_sequence(0, 1, pos);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fibonacci_10_first() {
        let vector = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, &elem) in vector.iter().enumerate() {
            assert_eq!(fibonacci(i as u32), elem);
        }
    }

    #[test]
    fn test_lucas_10_first() {
        let vector = vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76];
        for (i, &elem) in vector.iter().enumerate() {
            assert_eq!(lucas(i as u32), elem);
        }
    }
}
