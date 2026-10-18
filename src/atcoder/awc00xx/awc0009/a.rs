// https://atcoder.jp/contests/awc0009/tasks/awc0009_a

fn run(_k: usize, m: usize, l: Vec<usize>) -> usize {
    l.into_iter().sum::<usize>() % m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0009_a() {
        let tests = [
            TestCase(3, 100, vec![150, 80, 45], 75),
            TestCase(5, 1000, vec![1200, 350, 890, 460, 720], 620),
            TestCase(10, 500000000, vec![9999, 8765, 4321, 5678, 1234, 7777, 3333, 6666, 2222, 8888], 58883),
        ];

        for TestCase(k, m, l, expected) in tests {
            assert_eq!(run(k, m, l), expected);
        }
    }
}
