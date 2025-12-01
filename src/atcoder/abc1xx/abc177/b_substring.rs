// https://atcoder.jp/contests/abc177/tasks/abc177_b

fn run(s: &str, t: &str) -> usize {
    let mut ans = std::usize::MAX;

    let ss: Vec<char> = s.chars().collect();
    let tt: Vec<char> = t.chars().collect();

    for i in 0..=(ss.len() - tt.len()) {
        let mut differ_count = 0;

        for j in 0..tt.len() {
            if ss[i+j] != tt[j] {
                differ_count += 1;
            }
        }

        ans = ans.min(differ_count);
    }

    ans
}

fn run2(s: &str, t: &str) -> usize {
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    let t_len = t.len();

    s_vec.windows(t_len)
        .map(|a| {
            a.into_iter().zip(t_vec.iter())
                .filter(|t| {
                    t.0 != t.1
                })
                .count()
            })
            .min()
            .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize);

    #[test]
    fn abc177_b() {
        let tests = [
            TestCase("cabacc", "abc", 1),
            TestCase("codeforces", "atcoder", 6),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
            assert_eq!(run2(s, t), expected);
        }
    }
}
