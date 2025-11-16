// https://atcoder.jp/contests/abc153/tasks/abc153_a

fn run(hp: usize, a: usize) -> usize {
    if hp % a == 0 {
        hp / a
    } else {
        hp / a + 1
    }
}

fn run2(hp: usize, a: usize) -> usize {
    (hp as f64 / a as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc153_a() {
        let tests = [
            TestCase(10, 4, 3),
            TestCase(10000, 1, 10000),
            TestCase(1, 10000, 1),
        ];

        for TestCase(h, a, expected) in tests {
            assert_eq!(run(h, a), expected);
            assert_eq!(run2(h, a), expected);
        }
    }
}
