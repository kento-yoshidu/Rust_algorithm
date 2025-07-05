// https://atcoder.jp/contests/abc120/tasks/abc120_b

fn run(a: usize, b: usize, k: usize) -> usize {
    let mut count = 0;
    let mut result = 0;

    for i in (1..=std::cmp::min(a, b)).rev() {
        if a % i == 0 && b % i == 0 {
            count += 1;

            if count == k {
                result = i;
                break
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc120_b() {
        let tests = [
            TestCase(8, 12, 2, 2),
            TestCase(100, 50, 4, 5),
            TestCase(1, 1, 1, 1),
        ];

        for TestCase(a, b, k, expected) in tests {
            assert_eq!(run(a, b, k), expected);
        }
    }
}
