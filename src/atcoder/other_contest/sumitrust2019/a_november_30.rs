// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_a

fn run(m1: usize, _d1: usize, m2: usize, _d2: usize) -> usize {
    if m1 != m2 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, 16, 11, 17, 0),
            TestCase(11, 30, 12, 1, 1),
        ];

        for TestCase(m1, d1, m2, d2, expected) in tests {
            assert_eq!(run(m1, d1, m2, d2), expected);
        }
    }
}
