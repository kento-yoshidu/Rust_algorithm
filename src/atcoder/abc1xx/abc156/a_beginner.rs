// https://atcoder.jp/contests/abc156/tasks/abc156_a

pub fn run(n: usize, r: usize) -> usize {
    if n >= 10 {
        r
    } else {
        (10 - n) * 100 + r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3719, run(2, 2919));
        assert_eq!(3051, run(22, 3051));
    }
}
