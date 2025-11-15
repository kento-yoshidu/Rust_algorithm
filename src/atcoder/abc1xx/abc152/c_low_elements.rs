// https://atcoder.jp/contests/abc152/tasks/abc152_c

fn run(_n: usize, p: Vec<usize>) -> usize {
    let mut ans = 1;
    let mut min = p[0];

    for num in p {
        if num < min {
            min = num;
            ans += 1
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc152_c() {
        let tests = [
            TestCase(5, vec![4, 2, 5, 1, 3], 3),
            TestCase(4, vec![4, 3, 2, 1], 4),
            TestCase(6, vec![1, 2, 3, 4, 5, 6], 1),
            TestCase(8, vec![5, 7, 4, 2, 6, 8, 1, 3], 4),
            TestCase(1, vec![1], 1),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
