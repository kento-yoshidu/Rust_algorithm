// https://atcoder.jp/contests/abc031/tasks/abc031_b

fn run(l: isize, h: isize, _n: usize, v: Vec<isize>) -> Vec<isize> {
    v.into_iter()
        .map(|i| {
            if i < l {
                l - i
            } else if h < i {
                -1
            } else {
                0
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, usize, Vec<isize>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(300, 400, 3, vec![240, 350, 480], vec![60, 0, -1]),
            TestCase(50, 80, 5, vec![10000, 50, 81, 80, 0], vec![-1, 0, -1, 0, 50]),
        ];

        for TestCase(l, h, n, v, expected) in tests {
            assert_eq!(run(l, h, n, v), expected);
        }
    }
}
