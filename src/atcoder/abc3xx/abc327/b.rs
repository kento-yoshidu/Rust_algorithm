// https://atcoder.jp/contests/abc327/tasks/abc327_b

fn run(b: usize) -> isize {
    (1..=15)
        .find(|i| {
            (*i as usize).pow(*i as u32) == b
        })
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize);

    #[test]
    fn abc327_b() {
        let tests = [
            TestCase(27, 3),
            TestCase(100, -1),
            TestCase(10000000000, 10),
        ];

        for TestCase(b, expected) in tests {
            assert_eq!(run(b), expected);
        }
    }
}
