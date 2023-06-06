pub fn run(a: i32, b: i32, vec: Vec<i32>) -> i32 {
    let total = a + b;

    let index = vec.iter().position(|&x| x == total).unwrap() as i32;

    index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(125, 175, vec![200, 300, 400]));
        assert_eq!(1, run(1, 1, vec![2]));
        assert_eq!(5, run(123, 456, vec![135, 246, 357, 468, 579]));
    }
}
