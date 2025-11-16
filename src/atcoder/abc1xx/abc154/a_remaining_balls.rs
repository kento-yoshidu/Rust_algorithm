// https://atcoder.jp/contests/abc154/tasks/abc154_a

fn run(s: &str, _t: &str, a: usize, b: usize, u: &str) -> (usize, usize) {
    if s == u {
        (a-1, b)
    } else {
        (a, b-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize, usize, &'static str, (usize, usize));

    #[test]
    fn abc154_a() {
        let tests = [
            TestCase("red", "blue", 3, 4, "red", (2, 4)),
            TestCase("red", "blue", 5, 5, "blue", (5, 4)),
        ];

        for TestCase(s, t, a, b, u, expected) in tests {
            assert_eq!(run(s, t, a, b, u), expected);
        }
    }
}
