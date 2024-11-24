// https://atcoder.jp/contests/code-festival-2014-qualb/tasks/code_festival_qualB_b

fn run(_n: usize, k: usize, a: &Vec<usize>) -> usize {
    let mut sum = 0;

    for (i, n) in a.into_iter().enumerate() {
        sum += n;

        if sum >= k {
            return i+1;
        }
    }

    unreachable!();
}

fn func(sum: usize, i: usize, k: usize, a: &Vec<usize>) -> usize {
    if sum + a[i] >= k {
        i+1
    } else {
        func(sum+a[i], i+1, k, a)
    }
}

fn run2(_n: usize, k: usize, a: &Vec<usize>) -> usize {
    func(0, 0, k, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 100, vec![30, 10, 40, 10, 50, 10], 5),
            TestCase(6, 200, vec![100, 100, 100, 100, 100, 100], 2),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, &a), expected);
            assert_eq!(run2(n, k, &a), expected);
        }
    }
}
