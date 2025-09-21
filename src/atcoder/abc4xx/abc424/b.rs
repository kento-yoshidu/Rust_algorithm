// https://atcoder.jp/contests/abc424/tasks/abc424_b

fn run(n: usize, m: usize, _k: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut count = vec![0; n];

    for (a, _b) in ab {
        count[a-1] += 1;

        if count[a-1] == m {
            ans.push(a);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc424_b() {
        let tests = [
            TestCase(3, 2, 5, vec![(1, 1), (3, 2), (2, 1), (3, 1), (1, 2)], vec![3, 1]),
            TestCase(2, 2, 2, vec![(1, 1), (2, 2)], vec![]),
        ];

        for TestCase(n, m, k, ab, expected) in tests {
            assert_eq!(run(n, m, k, ab), expected);
        }
    }
}
