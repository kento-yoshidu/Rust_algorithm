#[allow(dead_code)]
pub fn run(s: String) -> String {
    if s == String::from("Hello,World!") {
        String::from("AC")
    } else {
        String::from("WA")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("AC"), run(String::from("Hello,World!")));
        assert_eq!(String::from("WA"), run(String::from("Hello,world!")));
        assert_eq!(String::from("WA"), run(String::from("Hello!World!")));
    }
}
