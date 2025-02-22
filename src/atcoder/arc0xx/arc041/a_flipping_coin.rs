// https://atcoder.jp/contests/arc041/tasks/arc041_a

fn run(x: usize, y: usize, k: usize) -> usize {
    if y >= k {
        x + k
    } else {
        x + y - (k - y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, 1, 4),
            TestCase(3, 2, 4, 3),
            TestCase(3, 2, 5, 2),
        ];

        for TestCase(x, y, k, expected) in tests {
            assert_eq!(run(x, y, k), expected);
        }
    }
}
