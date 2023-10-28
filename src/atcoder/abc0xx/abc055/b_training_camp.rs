// https://atcoder.jp/contests/abc055/tasks/abc055_b

pub fn run(n: usize) -> usize {
    (1..=n).fold(1, |state, i| { state * i % 1000000007 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(3));
        assert_eq!(3628800, run(10));
        assert_eq!(457992974, run(100000));
    }
}
