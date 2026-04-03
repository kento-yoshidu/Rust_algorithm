// https://atcoder.jp/contests/abc275/tasks/abc275_b

fn run(a: i128, b: i128, c: i128, d: i128, e: i128, f: i128) -> i128 {
    let m = 998244353;

    let x = (a % m) * (b % m) * (c % m);
    let y = (d % m) * (e % m) * (f % m);

    (x + m - y) % m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i128, i128, i128, i128, i128, i128, i128);
    #[test]
    fn abc275_b() {
        let tests = [
            TestCase(2, 3, 5, 1, 2, 4, 22),
            TestCase(1, 1, 1000000000, 0, 0, 0, 1755647),
            TestCase(1000000000000000000, 1000000000000000000, 1000000000000000000, 1000000000000000000, 1000000000000000000, 1000000000000000000, 0),
            TestCase(998244353, 998244353, 998244353, 1, 1, 1, 998244352),
        ];

        for TestCase(a, b, c ,d, e, f, expected) in tests {
            assert_eq!(run(a, b, c ,d , e, f), expected);
        }
    }
}
