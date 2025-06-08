// https://atcoder.jp/contests/abc072/tasks/arc082_b

fn run(n: usize, p: Vec<usize>) -> usize {
    let mut ans = 0;

    let mut vec = p.clone();

    for i in 0..n-1 {
        if vec[i] == i+1 {
            vec.swap(i, i+1);
            ans += 1;
        }
    }

    if *vec.last().unwrap() == n {
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 2),
            TestCase(4, vec![1, 2, 3, 4], 2),
            TestCase(5, vec![1, 4, 3, 5, 2], 2),
            TestCase(2, vec![1, 2], 1),
            TestCase(2, vec![2, 1], 0),
            TestCase(9, vec![1, 2, 4, 9, 5, 8, 7, 3, 6], 3),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
