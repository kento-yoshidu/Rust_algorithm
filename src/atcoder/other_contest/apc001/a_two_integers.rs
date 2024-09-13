// https://atcoder.jp/contests/apc001/tasks/apc001_a

fn run(x: usize, y: usize) -> isize {
    if x % y == 0 {
        -1
    } else {
        x as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 6, 8),
            TestCase(3, 3, -1),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
