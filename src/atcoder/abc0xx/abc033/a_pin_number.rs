pub fn run(n: String) -> String {
    let chars: Vec<char> = n.chars().collect();

    if chars[0] == chars[1] && chars[2] == chars[3] && chars[0] == chars[3] {
        String::from("SAME")
    } else {
        String::from("DIFFERENT")
    }
}

pub fn run2(n: String) -> String {
    use std::collections::HashSet;

    let map: HashSet<char> = n.chars().collect();

    if map.len() == 1 {
        String::from("SAME")
    } else {
        String::from("DIFFERENT")
    }
}

pub fn run3(n: String) -> String {
    let mut vec: Vec<char> = n.chars().collect();

    vec.dedup();

    if vec.len() == 1 {
        String::from("SAME")
    } else {
        String::from("DIFFERENT")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("SAME"), run(String::from("2222")));
        assert_eq!(String::from("DIFFERENT"), run(String::from("1221")));
        assert_eq!(String::from("SAME"), run(String::from("0000")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("SAME"), run2(String::from("2222")));
        assert_eq!(String::from("DIFFERENT"), run2(String::from("1221")));
        assert_eq!(String::from("SAME"), run2(String::from("0000")));
    }

    #[test]
    fn test3() {
        assert_eq!(String::from("SAME"), run3(String::from("2222")));
        assert_eq!(String::from("DIFFERENT"), run3(String::from("1221")));
        assert_eq!(String::from("SAME"), run3(String::from("0000")));
    }
}
