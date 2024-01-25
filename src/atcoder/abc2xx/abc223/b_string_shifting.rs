// https://atcoder.jp/contests/abc223/tasks/abc223_b

pub fn run(s: &str) -> (String, String) {
    let mut ans = Vec::new();

    for i in 0..s.len() {
        ans.push(format!("{}{}", &s[i..], &s[0..i]));
    }

    ans.sort();

    (ans[0].to_string(), ans[ans.len()-1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((String::from("aaab"), String::from("baaa")), run("aaba"));
        assert_eq!((String::from("z"), String::from("z")), run("z"));
        assert_eq!((String::from("aabracadabr"), String::from("racadabraab")), run("abracadabra"));
    }
}
