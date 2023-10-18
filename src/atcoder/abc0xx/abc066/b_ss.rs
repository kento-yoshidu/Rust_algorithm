// https://atcoder.jp/contests/abc066/tasks/abc066_b

fn check(s: &str) -> bool {
    if s[0..s.len()/2] == s[s.len()/2..] {
        true
    } else {
        false
    }
}

pub fn run(s: String) -> usize {
    (0..s.len())
        .rev()
        .skip(1)
        .step_by(2)
        .find(|i| {
            check(&s[0..*i])
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(String::from("abaababaab")));
        assert_eq!(2, run(String::from("xxxx")));
        assert_eq!(6, run(String::from("abcabcabcabc")));
        assert_eq!(14, run(String::from("akasakaakasakasakaakas")));
    }
}
