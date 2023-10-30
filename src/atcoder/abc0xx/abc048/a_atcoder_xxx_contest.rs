// https://atcoder.jp/contests/abc048/tasks/abc048_a

pub fn run(a: String, b: String, c: String) -> String {
    let vec = vec![a, b, c];

    let mut ans = String::new();

    vec.iter().for_each(|s| {
        ans.push(s.chars().nth(0).unwrap());
    });

    ans
}

pub fn run2(a: &str, b: &str, c: &str) -> String {
    let vec = vec![a, b, c];

    vec.iter()
        .map(|v| {
            v.chars().nth(0).unwrap()
        })
        .collect()
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

    #[test]
    fn test2() {
        assert_eq!(String::from("ABC"), run2("AtCoder", "Beginner", "Contest"));
        assert_eq!(String::from("ASC"), run2("AtCoder", "Snuke", "Contest"));
        assert_eq!(String::from("AXC"), run2("AtCoder", "X", "Contest"));
    }
}
