// https://atcoder.jp/contests/abc024/tasks/abc024_b

fn run(_n: usize, t: usize, a: &Vec<usize>) -> usize {
    let mut ans = 0;

    for num in a.windows(2) {
        if num[1] - num[0] > t {
            ans += t
        } else {
            ans += num[1] - num[0]
        }
    }

    ans + t
}

fn run2(n: usize, t: usize, a: &Vec<usize>) -> usize {
    (0..n-1).map(|i| {
        t.min(a[i+1] - a[i])
    }).sum::<usize>() + t
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 10, vec![20, 100, 105, 217, 314], 45),
            TestCase(10, 10, vec![1, 2 ,3, 4, 5, 6, 7, 8, 9, 10], 19),
            TestCase(10, 100000, vec![3, 31, 314, 3141, 31415, 314159, 400000, 410000, 500000, 777777], 517253),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, &a), expected);
        }
    }
}
