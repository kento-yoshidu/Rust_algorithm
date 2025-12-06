// https://yukicoder.me/problems/no/2789

fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
    let t = 31 - a + b;

    if c <= t {
        1
    } else if d <= t {
        2
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn yuki_2789() {
        let tests = [
            TestCase(25, 10, 20, 16, 2),
            TestCase(1, 1, 8, 5, 1),
            TestCase(1, 31, 100, 99, 3),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
