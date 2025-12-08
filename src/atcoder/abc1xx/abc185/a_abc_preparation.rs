// https://atcoder.jp/contests/abc185/tasks/abc185_a

fn run(arr: [usize; 4]) -> usize {
    arr.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 4], usize);

    #[test]
    fn abc185_a() {
        let tests = [
            TestCase([5, 3, 7, 11], 3),
            TestCase([100, 100, 1, 100], 1)
        ];

        for TestCase(arr, expected) in tests {
            assert_eq!(run(arr), expected);
        }
    }
}
