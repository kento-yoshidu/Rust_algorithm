
pub fn run(_n: usize, vec: Vec<usize>) -> Vec<usize> {
    vec.into_iter()
        .filter(|&i| i % 2 == 0)
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 6], run(5, vec![1, 2, 3, 5, 6]));
        assert_eq!(vec![2, 2, 2], run(5, vec![2, 2, 2, 3, 3]));
        assert_eq!(vec![22, 8, 30, 12, 14], run(10, vec![22, 3, 17, 8, 30, 15, 12, 14, 11, 17]));
    }
}
