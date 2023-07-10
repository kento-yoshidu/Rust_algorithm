// https://atcoder.jp/contests/abc177/tasks/abc177_b

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    assert_eq!(1, run(String::from("cabacc"), String::from("abc")));
    assert_eq!(6, run(String::from("codeforces"), String::from("atcoder")));
    }
}
