// https://atcoder.jp/contests/abc057/tasks/abc057_c

fn calc(n: usize) -> usize {
    let mut ans = 0;

    let mut num = n;

    loop {
        if num == 0 {
            break
        }

        ans += 1;
        num = num / 10;
    }

    ans
}

fn run(n: usize) -> usize {
    let mut ans = n;

    for i in 1..=n {
        if i*i > n {
            break
        }

        if n % i == 0 {
            // nの約数はiとj
            let j = n / i;

            // 約数の内、大きい方の桁数を求め、最も小さいものが答え
            ans = ans.min(calc(j.max(i)));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10000, 3),
            TestCase(1000003, 7),
            TestCase(9876543210, 6),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
