// https://atcoder.jp/contests/abc148/tasks/abc148_b

pub fn run(n: usize, s1: String, s2: String) -> String {
    let mut result = String::from("");

    for i in 0..n {
        result += format!("{}{}", s1.chars().nth(i).unwrap(), s2.chars().nth(i).unwrap()).as_str();
    }

    result
}

pub fn run2(_n: usize, s1: String, s2: String) -> String {
    s1.chars().zip(s2.chars()).map(|(c1, c2)| {
        format!("{}{}", c1, c2)
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("icpc"), run(2, String::from("ip"), String::from("cc")));
        assert_eq!(String::from("humuhumunukunuku"), run(8, String::from("hmhmnknk"), String::from("uuuuuuuu")));
        assert_eq!(String::from("aaaaaaaaaa"), run(5, String::from("aaaaa"), String::from("aaaaa")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("icpc"), run2(2, String::from("ip"), String::from("cc")));
        assert_eq!(String::from("humuhumunukunuku"), run2(8, String::from("hmhmnknk"), String::from("uuuuuuuu")));
        assert_eq!(String::from("aaaaaaaaaa"), run2(5, String::from("aaaaa"), String::from("aaaaa")));
    }
}
