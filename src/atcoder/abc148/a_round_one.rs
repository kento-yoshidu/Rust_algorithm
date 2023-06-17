#[allow(dead_code)]
pub fn run(u: usize, s1: String, s2: String) -> String {
    let mut result = String::from("");

    for i in 0..u {
        result += format!("{}{}", s1.chars().nth(i).unwrap(), s2.chars().nth(i).unwrap()).as_str();
    }

    result
}

#[allow(dead_code)]
pub fn run2(s1: String, s2: String) -> String {
    let mut ans = String::new();

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        ans.push(c1);
        ans.push(c2);
    }

    ans
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
}
