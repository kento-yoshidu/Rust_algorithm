// https://atcoder.jp/contests/abc052/tasks/abc052_a

pub fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
    if a*b >= c*d {
        a*b
    } else {
        c*d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, 2, 7, 15),
            TestCase(100, 600, 200, 300, 60000),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
