fn fib_sequence(first: u64, second: u64, position: u64) -> u64 {
    let mut vector = vec![first, second];
    for _ in 0..position {
        (vector[0], vector[1]) = (vector[1], vector[0] + vector[1]);
    }
    return vector[0];
}

pub fn fibonacci(position: u64) -> u64 {
    return fib_sequence(0, 1, position);
}

pub fn lucas(position: u64) -> u64 {
    return fib_sequence(2, 1, position);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fibonacci_10_first() {
        let vector = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, &elem) in vector.iter().enumerate() {
            assert_eq!(fibonacci(i as u64), elem);
        }
    }

    #[test]
    fn test_lucas_10_first() {
        let vector = vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76];
        for (i, &elem) in vector.iter().enumerate() {
            assert_eq!(lucas(i as u64), elem);
        }
    }
}
