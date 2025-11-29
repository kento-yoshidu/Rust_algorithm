// https://atcoder.jp/contests/abc176/tasks/abc176_c

fn run(_n: usize, a: Vec<isize>) -> isize {
    a.into_iter()
        .scan(0, |state, num| {
            *state = num.max(*state);
            Some(*state - num)
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc176_c() {
        let tests = [
            TestCase(5, vec![2, 1, 5, 4, 3], 4),
            TestCase(0, vec![3, 3, 3, 3, 3], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
