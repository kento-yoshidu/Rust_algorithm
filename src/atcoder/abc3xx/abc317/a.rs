// https://atcoder.jp/contests/abc317/tasks/abc317_a

fn run (_n: usize, h: usize, x: usize, p: Vec<usize>) -> usize {
    p.into_iter()
        .position(|num| {
            num >= (x - h)
        })
        .unwrap()+1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn abc317_a() {
        let tests = [
            TestCase(3, 100, 200, vec![50, 200, 999], 2),
            TestCase(2, 10, 21, vec![10, 999], 2),
            TestCase(10, 500, 999, vec![38, 420, 490, 585, 613, 614, 760, 926, 945, 999], 4),
        ];

        for TestCase(n, h, x, p, expected) in tests {
            assert_eq!(run(n, h, x, p), expected);
        }
    }
}
