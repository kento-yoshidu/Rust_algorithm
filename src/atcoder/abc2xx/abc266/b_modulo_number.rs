// https://atcoder.jp/contests/abc266/tasks/abc266_b

fn run(n: isize) -> isize {
    let ans = n % 998244353;

    if ans < 0 {
        ans + 998244353
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn abc266_b() {
        let tests = [
            TestCase(998244354, 1),
            TestCase(-9982443534, 998244349),
            TestCase(-366387905869936605, 0),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
