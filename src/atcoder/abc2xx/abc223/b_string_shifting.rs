// https://atcoder.jp/contests/abc223/tasks/abc223_b

fn run(s: &str) -> (String, String) {
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

    struct TestCase(&'static str, (String, String));

    #[test]
    fn abc223_b() {
        let tests = [
            TestCase("aaba", ("aaab".to_string(), "baaa".to_string())),
            TestCase("z", ("z".to_string(), "z".to_string())),
            TestCase("abracadabra", ("aabracadabr".to_string(), "racadabraab".to_string())),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
