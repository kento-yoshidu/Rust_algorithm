// https://atcoder.jp/contests/abc351/tasks/abc351_a

fn run(a: Vec<usize>, b: Vec<usize>) -> usize {
    let a_total: usize = a.iter().sum();
    let b_total: usize = b.iter().sum();

    if b_total <= a_total {
        a_total - b_total + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![0, 1, 0, 1, 2, 2, 0, 0, 1], vec![1, 1, 0, 0, 0, 0, 1, 0], 5),
            TestCase(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
