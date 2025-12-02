// https://atcoder.jp/contests/abc234/tasks/abc234_a

fn f(x: usize) -> usize {
    x * x + 2 * x + 3
}

fn run(t: usize) -> usize {
    f(f(f(t) + t) + f(f(t)))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc234_a() {
        let tests = [
            TestCase(0, 1371),
            TestCase(3, 722502),
            TestCase(10, 1111355571),
        ];

        for TestCase(t, expected) in tests {
            assert_eq!(run(t), expected);
        }
    }
}
