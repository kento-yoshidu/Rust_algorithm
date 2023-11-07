// https://atcoder.jp/contests/abc049/tasks/abc049_a

pub fn run(c: &str) -> String {
    match c {
        "a" | "i" | "u" | "e" | "o" => String::from("vowel"),
        _ => String::from("consonant")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("vowel"), run("a"));
        assert_eq!(String::from("consonant"), run("z"));
        assert_eq!(String::from("consonant"), run("s"));
    }
}
