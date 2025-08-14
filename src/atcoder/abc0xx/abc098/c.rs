// https://atcoder.jp/contests/abc098/tasks/arc098_a

fn run(n: usize, s: &str) -> usize {
    let mut ans = std::usize::MAX;

    let mut w = vec![0; n+1];
    let mut e = vec![0; n+1];

    for (i, c) in s.chars().enumerate() {
        if c == 'W' {
            w[i] += 1;
        } else {
            e[i] += 1;
        }
    }

    for i in 1..=n {
        w[i] += w[i-1];
        e[i] += e[i-1];
    }

    for i in 0..n {
        let mut sum = 0;

        if i != 0 {
            sum += w[i-1];
        }

        sum += e[n-1] - e[i];

        ans = ans.min(sum);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn abc098_c() {
        let tests = [
            TestCase(5, "WEEWW", 1),
            TestCase(12, "WEWEWEEEWWWE", 4),
            TestCase(8, "WWWWWEEE", 3),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
