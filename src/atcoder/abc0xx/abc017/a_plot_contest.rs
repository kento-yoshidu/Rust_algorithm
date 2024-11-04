// https://atcoder.jp/contests/abc017/tasks/abc017_1

fn run(vec: &Vec<(usize, usize)>) -> usize {
    vec.into_iter()
        .map(|v| {
            v.0 as f64 * (v.1 as f64) * 0.1
        })
        .sum::<f64>() as usize
}

fn run2(vec: &Vec<(usize, usize)>) -> usize {
    vec.into_iter()
        .map(|t| {
            t.0 * t.1
        })
        .sum::<usize>() / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![(50, 7), (40, 8), (30, 9)], 94),
            TestCase(vec![(990, 10), (990, 10), (990, 10)], 2970),
        ];

        for TestCase(vec, expected) in tests {
            assert_eq!(run(&vec), expected);
            assert_eq!(run2(&vec), expected);
        }
    }
}
