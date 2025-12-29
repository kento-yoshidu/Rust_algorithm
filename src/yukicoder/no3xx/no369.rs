// https://yukicoder.me/problems/no/369

fn run(_n: usize, a: Vec<isize>, v: isize) -> isize {
    a.into_iter().sum::<isize>() - v
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize, isize);

    #[test]
    fn yuki_369() {
        let tests = [
            TestCase(4, vec![5, 1, 4, 3], 10, 3),
            TestCase(3, vec![2, 1, -1], 0, 2),
            TestCase(3, vec![-5, -2, -4], 0, -11),
        ];

        for TestCase(n, a, v, expected) in tests {
            assert_eq!(run(n, a, v), expected);
        }
    }
}
