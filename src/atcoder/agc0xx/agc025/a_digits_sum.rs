// https://atcoder.jp/contests/agc025/tasks/agc025_a

pub fn run(n: usize) -> usize {
    if n % 10 == 0 {
        10
    } else {
        let s = n.to_string();

        s.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as usize)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(15, 6),
            TestCase(100000, 10),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
