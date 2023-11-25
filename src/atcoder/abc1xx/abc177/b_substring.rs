// https://atcoder.jp/contests/abc177/tasks/abc177_b

pub fn run(s: String, t: String) -> usize {
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

pub fn run2(s: &str, t: &str) -> usize {
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    let t_len = t.len();

    s_vec.windows(t_len)
        .map(|a| {
            a.iter().zip(t_vec.iter())
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

    #[test]
    fn test() {
        assert_eq!(1, run(String::from("cabacc"), String::from("abc")));
        assert_eq!(6, run(String::from("codeforces"), String::from("atcoder")));
    }

    #[test]
    fn test2() {
        assert_eq!(1, run2("cabacc", "abc"));
        assert_eq!(6, run2("codeforces", "atcoder"));
    }
}
