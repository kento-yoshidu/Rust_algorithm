// https://atcoder.jp/contests/abc318/tasks/abc318_b

fn run(_n: usize, a: Vec<Vec<usize>>) -> usize {
    let mut filed = vec![vec![false; 101]; 101];

    for v in a {
        for x in v[0]..v[1] {
            for y in v[2]..v[3] {
                filed[x][y] = true
            }
        }
    }

    filed
        .into_iter()
        .map(|vec| {
            vec.iter().filter(|v| {
                **v
            }).count()
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![vec![0, 5, 1, 3], vec![1, 4, 0, 5], vec![2, 5, 2, 4]], 20),
            TestCase(2, vec![vec![0, 100, 0, 100], vec![0, 100, 0, 100]], 10000),
            TestCase(3, vec![vec![0, 1, 0, 1], vec![0, 3, 0, 5], vec![5, 10, 0, 10]], 65),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
