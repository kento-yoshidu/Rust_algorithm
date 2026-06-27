// https://atcoder.jp/contests/abc305/tasks/abc305_a

fn run(n: usize) -> usize {
    let station = n / 5 * 5;

    if n - station < 3 {
        station
    } else {
        station + 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc305_a() {
        let tests = [
            TestCase(53, 55),
            TestCase(21, 20),
            TestCase(100, 100),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
