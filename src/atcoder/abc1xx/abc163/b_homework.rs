// https://atcoder.jp/contests/abc163/tasks/abc163_b

fn run(n: usize, _m: usize, a: Vec<usize>) -> isize {
    let sum = a.into_iter().sum();

    if n >= sum {
        (n - sum) as isize
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, isize);

    #[test]
    fn abc163_b() {
        let tests = [
            TestCase(41, 2, vec![5, 6], 30),
            TestCase(10, 2, vec![5, 6], -1),
            TestCase(11, 2, vec![5, 6], 0),
            TestCase(314, 15, vec![9, 26, 5, 35, 8, 9, 79, 3, 23, 8, 46, 2, 6, 43, 3], 9),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
