// https://atcoder.jp/contests/abc003/tasks/abc003_1

fn run(n: usize) -> usize {
    let mut sum = 0_f64;

    for i in 1..=n {
        sum += i as f64 * 10000_f64;
    }

    sum as usize / n
}

fn run2(n: usize) -> usize {
    (1..=n).map(|num| {
        num * 10000
    }).sum::<usize>() / n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 35000),
            TestCase(91, 460000),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
