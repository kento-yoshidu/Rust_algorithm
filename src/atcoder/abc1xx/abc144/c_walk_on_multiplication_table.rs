// https://atcoder.jp/contests/abc144/tasks/abc144_c

fn run(n: usize) -> usize {
    let mut ans = n;

    for i in 1..n {
        if i*i > n {
            break
        }

        if n % i == 0 {
            let j = n / i;

            // (1, 1)からスタートするため、2を引く
            let num = i+j-2;

            ans = ans.min(num);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc144_c() {
        let tests = [
            TestCase(10, 5),
            TestCase(50, 13),
            TestCase(10000000019, 10000000018),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
