// https://atcoder.jp/contests/abc215/tasks/abc215_b

fn run(n: u128) -> u32 {
    let mut ans = 1;

    loop {
        if 2_u128.pow(ans) > n {
            return ans - 1;
        } else {
            ans += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u128, u32);

    #[test]
    fn abc215_b() {
        let tests = [
            TestCase(6, 2),
            TestCase(1, 0),
            TestCase(1000000000000000000, 59),
            TestCase(576460752303423488, 59),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
