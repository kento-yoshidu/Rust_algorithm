// https://atcoder.jp/contests/abc014/tasks/abc014_3

fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut vec: Vec<isize> = vec![0; 1_000_000+2];

    for (a, b) in ab {
        vec[a] += 1;
        vec[b+1] -= 1;
    }

    let mut ans = vec![vec[0]];

    for i in 1..vec.len() {
        ans.push(ans[i-1] + vec[i]);
    }

    ans.into_iter()
        .max()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(0, 2), (2, 3), (2, 4), (5, 6)], 3),
            TestCase(4, vec![(1000000, 1000000), (1000000, 1000000), (0, 1000000), (1, 1000000)], 4),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
