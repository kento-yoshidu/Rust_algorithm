#[allow(dead_code)]
pub fn run(s: String, t: String) -> String {
    if s.replace("0", "o").replace("1", "l") == t.replace("0", "o").replace("1", "l") {
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
        assert_eq!(String::from("Yes"), run(String::from("l0w"), String::from("1ow")));
        assert_eq!(String::from("No"), run(String::from("abc"), String::from("abr")));
        assert_eq!(String::from("Yes"), run(String::from("nok0"), String::from("n0ko")));
    }
}
