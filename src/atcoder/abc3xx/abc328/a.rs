// https://atcoder.jp/contests/abc328/tasks/abc328_a

fn run(_n: usize, x: usize, s: Vec<usize>) -> usize {
    s.into_iter()
        .filter(|num| *num <= x)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc328_a() {
        let tests = [
            TestCase(6, 200, vec![100, 675, 201, 200, 199, 328], 499),
            TestCase(8, 675, vec![675, 675, 675, 675, 675, 675, 675, 675], 5400),
            TestCase(8, 674, vec![675, 675, 675, 675, 675, 675, 675, 675], 0),
        ];

        for TestCase(n, x, s, expected) in tests {
            assert_eq!(run(n, x, s), expected);
        }
    }
}
