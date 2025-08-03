// https://atcoder.jp/contests/abc121/tasks/abc121_b

fn run(_n: isize, _m: isize, c: isize, b: &Vec<isize>, vec: &Vec<Vec<isize>>) -> usize {
    let mut ans = 0;

    for v in vec.iter() {
        let mut total = 0;

        for (i, num) in v.iter().enumerate() {
            total += num * b[i];
        }

        if (total as isize + c) > 0 {
            ans += 1;
        }
    }

    ans
}

fn run2(_n: isize, _m: isize, c: isize, n: &Vec<isize>, vec: &Vec<Vec<isize>>) -> usize {
    vec.into_iter()
        .filter(|v| {
            let total: isize = v.iter()
                .enumerate()
                .map(|(i, num)| {
                    num * n[i]
                })
                .sum();

            total + c > 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, Vec<isize>, Vec<Vec<isize>>, usize);

    #[test]
    fn abc121_b() {
        let tests = [
            TestCase(2, 3, -10, vec![1, 2, 3], vec![vec![3, 2, 1], vec![1, 2, 2]], 1),
            TestCase(5, 2, -4, vec![-2, 5], vec![vec![100, 41], vec![100, 40], vec![-3, 0], vec![-6, -2], vec![18, -13]], 2),
            TestCase(3, 3, 0, vec![100, -100, 0], vec![vec![0, 100, 100], vec![100, 100, 100], vec![-100, 100, 100]], 0),
        ];

        for TestCase(_n, m, c, n, vec, expected) in tests {
            assert_eq!(run(_n, m, c, &n, &vec), expected);
            assert_eq!(run2(_n, m, c, &n, &vec), expected);
        }
    }
}
