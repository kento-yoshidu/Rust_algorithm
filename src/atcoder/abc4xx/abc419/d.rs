// https://atcoder.jp/contests/abc419/tasks/abc419_d

fn run(n: usize, _m: usize, s: &str, t: &str, lr: Vec<(usize, usize)>) -> String {
    let mut imos = vec![0; n+2];

    for (l, r) in lr {
        imos[l] += 1;
        imos[r+1] -= 1;
    }

    let mut acc = vec![imos[0]];

    for i in 1..imos.len() {
        acc.push(imos[i] + acc[i-1]);
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut ans = Vec::new();

    for i in 1..=n {
        if acc[i] % 2 == 0 {
            ans.push(s[i-1]);
        } else {
            ans.push(t[i-1]);
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc419_d() {
        let tests = [
            TestCase(5, 3, "apple", "lemon", vec![(2, 4), (1, 5), (5, 5)], "lpple"),
            TestCase(10, 5, "lemwrbogje", "omsjbfggme", vec![(5, 8), (4, 8), (1, 3), (6, 6), (1, 4)], "lemwrfogje"),
        ];

        for TestCase(n, m, s, t, lr, expected) in tests {
            assert_eq!(run(n, m, s, t, lr), expected);
        }
    }
}
