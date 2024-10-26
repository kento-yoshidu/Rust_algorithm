// https://atcoder.jp/contests/joi2022yo1c/tasks/joi2022_yo1c_d

fn run(n: usize, _m: usize, xy: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans: Vec<usize> = (1..=n).collect();

    for (x, y) in xy {
        ans[x-1] = *y;
    }

    ans
}

fn run2(n: usize, _m: usize, xy: &Vec<(usize, usize)>) -> Vec<usize> {
    xy.into_iter()
        .fold((1..=n).collect(), |mut acc, (x, y)| {
            acc[x-1] = *y;

            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, vec![(1, 2), (3, 2), (2, 1), (1, 3)], vec![3, 1, 2]),
            TestCase(3, 3, vec![(1, 1), (2, 2), (3, 3)], vec![1, 2, 3]),
            TestCase(4, 2, vec![(1, 3), (2, 4)], vec![3, 4, 3, 4]),
            TestCase(4, 8, vec![(1, 3), (3, 2), (2, 4), (2, 3), (4, 1), (2, 1), (1, 4), (3, 3)], vec![4, 1, 3, 1]),
        ];

        for TestCase(n, m, xy, expected) in tests {
            assert_eq!(run(n, m, &xy), expected);
            assert_eq!(run2(n, m, &xy), expected);
        }
    }
}
