// https://atcoder.jp/contests/abc462/tasks/abc462_b

fn run(n: usize, ka: Vec<(usize, Vec<usize>)>) -> Vec<(usize, Vec<usize>)> {
    let mut ans = vec![vec![]; n];

    for (i, (_, a)) in ka.into_iter().enumerate() {
        let i = i + 1;

        for num in a {
            ans[num - 1].push(i);
        }
    }

    ans.into_iter()
        .map(|vec| {
            (vec.len(), vec)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Vec<usize>)>, Vec<(usize, Vec<usize>)>);

    #[test]
    fn abc462_b() {
        let tests = [
            TestCase(4, vec![(1, vec![2]), (1, vec![3]), (1, vec![2]), (3, vec![1, 2, 3])], vec![(1, vec![4]), (3, vec![1, 3, 4]), (2, vec![2, 4]), (0, vec![])]),
        ];

        for TestCase(n, ka, expected) in tests {
            assert_eq!(run(n, ka), expected);
        }
    }
}
