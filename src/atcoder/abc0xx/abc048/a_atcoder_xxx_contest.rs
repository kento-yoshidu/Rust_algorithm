#[allow(dead_code)]
pub fn run(a: String, b: String, c: String) -> String {
    let vec = vec![a, b, c];

    let mut ans = String::new();

    vec.iter().for_each(|s| {
        ans.push(s.chars().nth(0).unwrap());
    });

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(String::from("AtCoder"), String::from("Beginner"), String::from("Contest")));
        assert_eq!(String::from("ASC"), run(String::from("AtCoder"), String::from("Snuke"), String::from("Contest")));
        assert_eq!(String::from("AXC"), run(String::from("AtCoder"), String::from("X"), String::from("Contest")));
    }
}
