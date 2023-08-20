// https://atcoder.jp/contests/abc315/tasks/abc315_a

pub fn run(s: String) -> String {
    s.chars().filter(|c| {
        !"aiueo".contains(*c)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("tcdr"), run(String::from("atcoder")));
        assert_eq!(String::from("xyz"), run(String::from("xyz")));
        assert_eq!(String::from("bbbbcccc"), run(String::from("aaaabbbbcccc")));
    }
}
