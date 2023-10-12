// https://atcoder.jp/contests/abc034/tasks/abc034_b

fn run(n: usize) -> usize {
    if n % 2 == 0 {
        n - 1
    } else {
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(99, run(100));
        assert_eq!(123456790, run(123456789));
    }
}
