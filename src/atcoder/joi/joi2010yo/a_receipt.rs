// https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_a

fn run(a: [usize; 10]) -> usize {
    a[0] - a[1..].iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 10], usize);

    #[test]
    fn test() {
        let tests = [
            TestCase([9850, 1050, 800, 420, 380, 600, 820, 2400, 1800, 980], 600)
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
