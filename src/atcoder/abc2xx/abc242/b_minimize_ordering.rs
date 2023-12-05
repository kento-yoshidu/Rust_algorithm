// https://atcoder.jp/contests/abc242/tasks/abc242_b

pub fn run(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("aab"), run("aba"));
        assert_eq!(String::from("zzzz"), run("zzzz"));
    }
}
