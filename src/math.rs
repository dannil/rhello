pub fn fibonacci(pos: u32) -> u32 {
    let mut vector = vec![0, 1];
    for _ in 0..pos {
        let temp: u32 = vector[1];
        vector[1] = vector[0] + vector[1];
        vector[0] = temp;
    }
    return vector[0];
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
}
