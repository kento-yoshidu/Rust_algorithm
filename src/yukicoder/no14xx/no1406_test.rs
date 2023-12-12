// https://yukicoder.me/problems/no/1406

pub fn run(n: usize, a: Vec<usize>) -> usize {
    (0..=100)
        .filter(|num| {
            (a.iter().sum::<usize>() + num) % n == 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(25, run(4, vec![1, 2, 3]));
        assert_eq!(20, run(5, vec![87, 34, 2, 46]));
    }
}
