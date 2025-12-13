// https://atcoder.jp/contests/abc194/tasks/abc194_b

fn run(n: usize, a: Vec<(usize, usize)>) -> usize {
    let mut ans = std::usize::MAX;

    for i in 0..n {
        for j in 0..n {
            if i == j {
                ans = ans.min(a[i].0 + a[i].1);
            } else {
                ans = ans.min(a[i].0.max(a[j].1));
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(8, 5), (4, 4), (7, 9)], 5),
            TestCase(3, vec![(11, 7), (3, 2), (6, 7)], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
