// https://atcoder.jp/contests/abc417/tasks/abc417_b

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut a = a.clone();

    for num in b {
        if let Some(i) = a.iter().position(|x| *x == num) {
            a.remove(i);
        }
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn abc417_b() {
        let tests = [
            TestCase(8, 5, vec![1, 2, 2, 3, 3, 3, 5, 6], vec![2, 2, 7, 3, 2], vec![1, 3, 3, 5, 6]),
            TestCase(1, 2, vec![1], vec![1, 1], vec![]),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
