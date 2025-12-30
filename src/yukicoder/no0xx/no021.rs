// https://yukicoder.me/problems/no/21

fn run(_n: usize, _k: usize, a: Vec<usize>) -> usize {
    let min = *a.iter().min().unwrap();
    let max = *a.iter().max().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn yuki_021() {
        let tests = [
            TestCase(5, 3, vec![555, 20, 432, 301, 21], 535),
            TestCase(8, 4, vec![329, 980, 656, 738, 739, 542, 873, 501], 651),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
