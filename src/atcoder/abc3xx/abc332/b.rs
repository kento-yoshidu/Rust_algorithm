// https://atcoder.jp/contests/abc332/tasks/abc332_b

fn run(k: usize, g: usize, m: usize) -> (usize, usize) {
    (0..k)
        .fold((0, 0), |(glass, mug), _| {
            if glass == g {
                (0, mug)
            } else if mug == 0 {
                (glass, m)
            } else {
                if g < glass + mug {
                    (g, mug - (g - glass))
                } else {
                    (glass + mug, 0)
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, (usize, usize));

    #[test]
    fn abc332_a() {
        let tests = [
            TestCase(5, 300, 500, (200, 500)),
            TestCase(5, 100, 200, (0, 0)),
        ];

        for TestCase(k, g, m, expected) in tests {
            assert_eq!(run(k, g, m), expected);
        }
    }
}
