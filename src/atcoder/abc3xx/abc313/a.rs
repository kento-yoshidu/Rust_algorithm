// https://atcoder.jp/contests/abc313/tasks/abc313_a

fn run(n: usize, p: Vec<usize>) -> usize {
    if n == 1 {
        return 0
    }

    let max = p.iter().skip(1).max().unwrap();

    if p[0] > *max {
        0
    } else {
        max - p[0] + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc313_a() {
        let tests = [
            TestCase(4, vec![5, 15, 2, 10], 11),
            TestCase(4, vec![15, 5, 2, 10], 0),
            TestCase(3, vec![100, 100, 100], 1),
            TestCase(1, vec![1], 0),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
