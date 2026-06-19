// https://atcoder.jp/contests/abc299/tasks/abc299_b

fn calc(num: usize, c: Vec<usize>, r: Vec<usize>) -> usize {
    c.iter()
        .zip(r.iter())
        .enumerate()
        .filter(|(_, (l, _))| {
            **l == num
        })
        .max_by(|a, b| {
            a.1.1.cmp(b.1.1)
        })
        .map(|(i, (_, _))| i)
        .unwrap() + 1
}

fn run(_n: usize, t: usize, c: Vec<usize>, r: Vec<usize>) -> usize {
    if c.contains(&t) {
        calc(t, c, r)
    } else {
        calc(c[0], c, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn abc299_c() {
        let tests = [
            TestCase(4, 2, vec![1, 2, 1, 2], vec![6, 3, 4, 5], 4),
            TestCase(4, 2, vec![1, 3, 1, 4], vec![6, 3, 4, 5], 1),
            TestCase(2, 1000000000, vec![1000000000, 1], vec![1, 1000000000], 1),
        ];

        for TestCase(n, t, c, r, expected) in tests {
            assert_eq!(run(n, t, c, r), expected);
        }
    }
}
