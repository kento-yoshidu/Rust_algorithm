// https://atcoder.jp/contests/abc131/tasks/abc131_b

fn run(n: isize, l: isize) -> isize {
    let abs_min = (0..n).map(|i| {
        (l + i).abs()
    })
    .min()
    .unwrap();

    (0..n)
        .map(|i| {
            l + i
        })
        .filter(|num| {
            num.abs() != abs_min
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn abc131_b() {
        let tests = [
            TestCase(5, 2, 18),
            TestCase(3, -1, 0),
            TestCase(30, -50, -1044),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
