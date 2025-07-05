// https://atcoder.jp/contests/abc118/tasks/abc118_b

fn run(_n: usize, m: usize, v: Vec<Vec<usize>>) -> usize {
    let vec: Vec<Vec<usize>> = v.iter()
        .map(|vec| {
            vec.iter()
                .enumerate()
                .filter(|(i, _)| *i != 0)
                .map(|(_, v)| *v)
                .collect()
        })
        .collect();

    (1..=m)
        .filter(|num| {
            vec.iter()
                .all(|vec| {
                    vec.contains(num)
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn abc118_b() {
        let tests = [
            TestCase(3, 4, vec![vec![2, 1, 3], vec![3, 1, 2, 3], vec![2, 3, 2]], 1),
            TestCase(5, 5, vec![vec![4, 2, 3, 4, 5], vec![4, 1, 3, 4, 5], vec![4, 1, 2, 4, 5], vec![4, 1, 2, 3, 5], vec![4, 1, 2, 3, 4,]], 0),
            TestCase(1, 30, vec![vec![3, 5, 10, 30]], 3),
        ];

        for TestCase(n, m, v, expected) in tests {
            assert_eq!(run(n, m, v), expected);
        }
    }
}
