// https://atcoder.jp/contests/abc170/tasks/abc170_a

fn run(x: &Vec<usize>) -> usize {
    x.into_iter()
        .enumerate()
        .find(|(_, n)| {
            **n == 0
        })
        .unwrap().0 + 1
}

fn run2(x: &Vec<usize>) -> usize {
    15 - x.into_iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![0, 2, 3, 4, 5], 1),
            TestCase(vec![1, 0, 3, 4, 5], 2),
            TestCase(vec![1, 2, 0, 4, 5], 3),
            TestCase(vec![1, 2, 3, 0, 5], 4),
            TestCase(vec![1, 2, 3, 4, 0], 5),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(&x), expected);
            assert_eq!(run2(&x), expected);
        }
    }
}
