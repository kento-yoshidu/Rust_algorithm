// https://atcoder.jp/contests/abc241/tasks/abc241_a

fn run(a: [usize; 10]) -> usize {
    let mut ans = 0;

    for _ in 0..3 {
        ans = a[ans];
    }

    ans
}

fn run2(a: [usize; 10]) -> usize {
    a[a[a[0]]]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 10], usize);

    #[test]
    fn abc241_a() {
        let tests = [
            TestCase([9, 0, 1, 2, 3, 4, 5, 6, 7, 8], 7),
            TestCase([4, 8, 8, 8, 0, 8, 8, 8, 8, 8], 4),
            TestCase([0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
            assert_eq!(run2(a), expected);
        }
    }
}
