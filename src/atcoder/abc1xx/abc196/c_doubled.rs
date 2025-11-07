// https://atcoder.jp/contests/abc196/tasks/abc196_c

fn func(n: usize) -> usize {
    let mut num = 1;
    let mut nn= n;

    loop {
        num *= 10;
        nn /= 10;

        if nn == 0 {
            break
        }
    }

    n * num + n
}

fn run(n: usize) -> usize {
    let mut ans = 0;

    let mut i = 1;

    loop {
        let num = func(i);

        if num <= n {
            ans += 1;
            i += 1;
        } else {
            break
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc196_c() {
        let tests = [
            TestCase(33, 3),
            TestCase(1333, 13),
            TestCase(10000000, 999),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
