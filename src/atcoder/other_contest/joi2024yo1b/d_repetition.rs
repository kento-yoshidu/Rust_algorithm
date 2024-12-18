// https://atcoder.jp/contests/joi2024yo1b/tasks/joi2024_yo1b_d

fn calc(state: usize, n: usize, count: usize) -> usize {
    if n <= state {
        count
    } else {
        match state % 3 {
            0 => calc(state+1, n, count+1),
            1 => calc(state*2, n, count+1),
            2 => calc(state*3, n, count+1),
            _ => unreachable!(),
        }
    }
}

fn run(x: usize, n: usize) -> usize {
    calc(x, n, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 40, 4),
            TestCase(3, 4, 1),
            TestCase(20, 62, 3),
            TestCase(1, 100000, 19),
        ];

        for TestCase(x, n, expected) in tests {
            assert_eq!(run(x, n), expected);
        }
    }
}
