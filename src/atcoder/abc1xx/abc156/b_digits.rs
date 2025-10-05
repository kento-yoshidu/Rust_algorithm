// https://atcoder.jp/contests/abc156/tasks/abc156_b

fn calc(n: usize, base: usize) -> u32 {
    if n < base {
        return 1;
    } else {
        let boxed = Box::new(n);

        calc(*boxed / base, base) + 1
    }
}

fn run(n: usize, k: usize) -> u32 {
    calc(n, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, u32);

    #[test]
    fn abc156_b() {
        let tests = [
            TestCase(11, 2, 4),
            TestCase(1010101, 10, 7),
            TestCase(314159265, 3, 18),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
