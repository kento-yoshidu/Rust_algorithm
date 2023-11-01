// https://atcoder.jp/contests/abc082/tasks/abc082_b

pub fn run(s: String, t: String) -> String {
    let mut ss: Vec<char> = s.chars().collect();
    ss.sort();

    let mut tt: Vec<char> = t.chars().collect();
    tt.sort();
    tt.reverse();

    if ss < tt {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(String::from("yx"), String::from("axy")));
        assert_eq!(String::from("Yes"), run(String::from("ratcode"), String::from("atlas")));
        assert_eq!(String::from("No"), run(String::from("cd"), String::from("abc")));
        assert_eq!(String::from("Yes"), run(String::from("w"), String::from("ww")));
        assert_eq!(String::from("No"), run(String::from("zzz"), String::from("zzz")));
    }
}
