// https://atcoder.jp/contests/abc195/tasks/abc195_c

fn run(n: isize) -> isize {
    let mut k = 1;
    let mut ans = 0;

    loop {
        if 1000_isize.pow(k) > n {
            return ans;
        }

        ans += n - (1000_isize.pow(k) - 1);
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1010, 11),
            TestCase(27182818284590, 107730272137364),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
