// https://atcoder.jp/contests/arc112/tasks/arc112_a

fn calc(a: usize, b: usize) -> usize {
    (a + b) * (b - a + 1) / 2
}

pub fn run(_t: usize, lr: Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans = Vec::new();

    for (l, r) in lr.into_iter() {
        let ll = l;
        let rr = r - l;

        if ll > rr {
            ans.push(0)
        } else {
            ans.push(calc(r - rr - l + 1, r - ll - l + 1));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![(2, 6), (0, 0), (1000000, 1000000), (12345, 67890), (0, 1000000)], vec![6, 1, 0, 933184801, 500001500001]),
        ];

        for TestCase(t, lr, expected) in tests {
            assert_eq!(run(t, lr), expected);
        }
    }
}
