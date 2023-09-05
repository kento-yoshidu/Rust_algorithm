// https://atcoder.jp/contests/abc132/tasks/abc132_a

pub fn run(s: String) -> String {
    let mut temp: Vec<_> = s.chars().collect();

    temp.sort();

    if temp[0] == temp[1] && temp[2] == temp[3] && temp[1] != temp[2] {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

pub fn run2(s: String) -> String {
    let mut vec: Vec<char> = s.chars().collect();

    vec.sort();
    vec.dedup();

    if vec.len() == 2 {
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
        assert_eq!(String::from("Yes"), run(String::from("ASSA")));
        assert_eq!(String::from("No"), run(String::from("STOP")));
        assert_eq!(String::from("Yes"), run(String::from("FFEE")));
        assert_eq!(String::from("No"), run(String::from("FREE")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(String::from("ASSA")));
        assert_eq!(String::from("No"), run2(String::from("STOP")));
        assert_eq!(String::from("Yes"), run2(String::from("FFEE")));
        assert_eq!(String::from("No"), run2(String::from("FREE")));
    }
}
