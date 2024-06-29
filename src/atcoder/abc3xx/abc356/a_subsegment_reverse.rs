// https://atcoder.jp/contests/abc356/tasks/abc356_a

fn run(n: usize, l: usize, r: usize) -> Vec<usize> {
    let mut ans: Vec<usize> = (1..=n).collect();

    for i in l..=r {
        ans[i-1] = l+r-i;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, 3, vec![1, 3, 2, 4, 5]),
            TestCase(7, 1, 1, vec![1, 2, 3, 4, 5, 6, 7]),
            TestCase(10, 1, 10, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
        ];

        for TestCase(n, l, r, expected) in tests {
            assert_eq!(run(n, l, r), expected);
        }
    }
}
