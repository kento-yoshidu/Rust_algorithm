// https://atcoder.jp/contests/abc148/tasks/abc148_d

fn run(n: usize, a: Vec<isize>) -> isize {
    if !a.contains(&1) {
        return -1;
    }

    let mut cur = 1;

    for i in 0..n {
        if a[i] == cur {
            cur += 1;
        }
    }

    (n as isize - cur + 1) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc148_d() {
        let tests = [
            TestCase(3, vec![2, 1, 2], 1),
            TestCase(3, vec![2, 2, 2], -1),
            TestCase(10, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3], 7),
            TestCase(1, vec![1], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
