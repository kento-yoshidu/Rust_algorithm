// https://atcoder.jp/contests/abc076/tasks/abc076_b

pub fn run(n: usize, k: usize) -> usize {
    (1..=n).fold(1, |sum, _| {
        if sum >= k {
            sum + k
        } else {
            sum * 2
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(4, 3));
        assert_eq!(76, run(10, 10));
    }
}
