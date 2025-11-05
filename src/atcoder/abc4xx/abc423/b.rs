// https://atcoder.jp/contests/abc423/tasks/abc423_b

fn run(n: usize, l: Vec<usize>) -> usize {
    let mut left = n;
    let mut right = 0;

    for i in 0..n {
        if l[i] == 1 {
            left = left.min(i);
            right = right.max(i);
        }
    }

    if left == n {
        0
    } else {
        right - left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc423_a() {
        let tests = [
            TestCase(5, vec![0, 1, 0, 0, 1], 3),
            TestCase(3, vec![1, 0, 1], 2),
            TestCase(8, vec![0, 0, 1, 1, 0, 1, 0, 0], 3),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
