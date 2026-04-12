// https://atcoder.jp/contests/abc453/tasks/abc453_b

fn run(_t: usize, x: isize, a: Vec<isize>) -> Vec<(usize, isize)> {
    let mut ans = Vec::new();

    let mut last = 0;

    for (i, t) in a.into_iter().enumerate() {
        if (t - last).abs() >= x {
            ans.push((i, t));
            last = t;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, Vec<(usize, isize)>);

    #[test]
    fn abc453_b() {
        let tests = [
            TestCase(6, 10, vec![30, 35, 40, 21, 30, 12, 31], vec![(0, 30), (2, 40), (3, 21), (6, 31)]),
        ];

        for TestCase(t, x, a, expected) in tests {
            assert_eq!(run(t, x, a), expected);
        }
    }
}
