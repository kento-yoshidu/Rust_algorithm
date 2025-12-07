// https://atcoder.jp/contests/abc435/tasks/abc435_c

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 1;

    for i in 0..n {
        if ans <= i {
            return ans;
        }

        ans = ans.max(i + a[i]);
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc435_c() {
        let tests = [
            TestCase(4, vec![3, 1, 4, 1], 4),
            TestCase(9, vec![1, 4, 1, 4, 2, 1, 3, 5, 6], 1),
            TestCase(10, vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
