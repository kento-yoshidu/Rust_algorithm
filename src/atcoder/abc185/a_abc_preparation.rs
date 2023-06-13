#[allow(dead_code)]
pub fn run(arr: [i32; 4]) -> i32 {
    *arr.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run([5, 3, 7, 11]));
        assert_eq!(1, run([100, 100, 1, 100]));
    }
}
