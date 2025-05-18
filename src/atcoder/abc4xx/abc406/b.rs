// https://atcoder.jp/contests/abc406/tasks/abc406_b

fn run(_n: usize, k: usize, a: Vec<usize>) -> u128 {
    let limit = 10_u128.pow(k as u32);

    a.iter()
        .fold(1_u128, |acc, n| {
            let prod = acc * (*n as u128);

            if prod >= limit {
                1
            } else {
                prod
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, u128);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, vec![7, 13, 3, 2, 5], 10),
            TestCase(2, 1, vec![2, 5], 1),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
