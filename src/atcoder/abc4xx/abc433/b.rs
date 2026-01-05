// https://atcoder.jp/contests/abc433/tasks/abc433_b

fn run(_n: usize, a: &Vec<usize>) -> Vec<isize> {
    let mut ans = Vec::new();

    for (i, x) in a.iter().enumerate() {
        let mut pos = -1;

        for (j, y) in a.iter().enumerate() {
            if i == j {
                break;
            }

            if y > x {
                pos = j as isize + 1;
            }
        }

        ans.push(pos);
    }

    ans
}

fn run2(n: usize, a: &Vec<usize>) -> Vec<isize> {
    (0..n)
        .map(|i| {
            a.iter()
                .take(i)
                .rposition(|&x| x > a[i])
                .map(|pos| (pos +1) as isize)
                .unwrap_or(-1)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<isize>);

    #[test]
    fn abc433_b() {
        let tests = [
            TestCase(4, vec![4, 3, 2, 5], vec![-1, 1, 2, -1]),
            TestCase(3, vec![7, 7, 7], vec![-1, -1, -1]),
            TestCase(6, vec![31, 9, 17, 10, 2, 9], vec![-1, 1, 1, 3, 4, 4]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
