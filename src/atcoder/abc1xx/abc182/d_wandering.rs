// https://atcoder.jp/contests/abc182/tasks/abc182_d

fn run(n: usize, a: Vec<isize>) -> isize {
    // aの累積和
    let mut s = Vec::new();

    // sの累積和
    let mut ss = Vec::new();

    for i in 0..n {
        if i == 0 {
            s.push(a[i]);
            ss.push(s[i]);
        } else {
            s.push(s[i-1] + a[i]);
            ss.push(ss[i-1] + s[i]);
        }
    }

    // s[i]の時、最も大きいものを保持
    let mut s_max = Vec::new();

    for i in 0..n {
        if i == 0 {
            s_max.push(s[0]);
        } else {
            let m = s[i].max(s_max[i-1]);

            s_max.push(m);
        }
    }

    let mut ans = 0;

    for (a, b) in ss.iter().zip(s_max.iter()) {
        ans = ans.max(a + b)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc182_d() {
        let tests = [
            TestCase(3, vec![2, -1, -2], 5),
            TestCase(5, vec![-2, 1, 3, -1, -1], 2),
            TestCase(5, vec![-1000, -1000, -1000, -1000, -1000], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
