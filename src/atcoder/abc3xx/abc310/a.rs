// https://atcoder.jp/contests/abc310/tasks/abc310_a

fn run(_x: usize, p: usize, q: usize, d: Vec<usize>) -> usize {
    let min = d.into_iter().min().unwrap();

    if q + min < p {
        q + min
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn abc310_a() {
        let tests = [
            TestCase(3, 100, 50, vec![60, 20, 40], 70),
            TestCase(3, 100, 50, vec![60000, 20000, 40000], 100),
        ];

        for TestCase(x, p, q, d, expected) in tests {
            assert_eq!(run(x, p, q, d), expected);
        }
    }
}
