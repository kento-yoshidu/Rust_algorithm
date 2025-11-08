// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_u

fn f(a: usize) -> usize {
    if a <= 1 {
        1
    } else {
        a * f(a-1)
    }
}

fn run(n: usize, r: usize) -> usize {
    f(n) / (f(r) * f(n-r))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn ma021() {
        let tests = [
            TestCase(6, 2, 15)
        ];

        for TestCase(n, r, expected) in tests {
            assert_eq!(run(n, r), expected);
        }
    }
}
