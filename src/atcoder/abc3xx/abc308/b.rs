// https://atcoder.jp/contests/abc308/tasks/abc308_b

fn run(n: usize, m: usize, c: &Vec<&str>, d: &Vec<&str>, p: &Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        if let Some(j) = (0..m).find(|&j| c[i] == d[j]) {
            ans += p[j + 1];
        } else {
            ans += p[0];
        }
    }

    ans
}

fn run2(n: usize, m: usize, c: &Vec<&str>, d: &Vec<&str>, p: &Vec<usize>) -> usize {
    (0..n)
        .fold(0, |state, i| {
            if let Some(j) = (0..m).find(|&j| c[i] == d[j]) {
                state + p[j + 1]
            } else {
                state + p[0]
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, Vec<usize>, usize);

    #[test]
    fn abc308_b() {
        let tests = [
            TestCase(3, 2, vec!["red", "green", "blue"], vec!["blue", "red"], vec![800, 1600, 2800], 5200),
            TestCase(3, 2, vec!["code", "queen", "atcoder"], vec!["king", "queen"], vec![10, 1, 1], 21),
        ];

        for TestCase(n, m, c, d, p, expected) in tests {
            assert_eq!(run(n, m, &c, &d, &p), expected);
            assert_eq!(run2(n, m, &c, &d, &p), expected);
        }
    }
}
