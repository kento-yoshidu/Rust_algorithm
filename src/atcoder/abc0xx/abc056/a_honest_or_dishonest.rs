// https://atcoder.jp/contests/abc056/tasks/abc056_a

#[allow(dead_code)]
pub fn run(a: &str, b: &str) -> String {
    if a == "H" {
        return b.to_string()
    }

    if b == "H" {
        "D".to_string()
    } else {
        "H".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("H"), run("H", "H"));
        assert_eq!(String::from("D"), run("D", "H"));
        assert_eq!(String::from("H"), run("D", "D"));
    }
}
