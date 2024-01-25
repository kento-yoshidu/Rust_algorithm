// https://atcoder.jp/contests/abc217/tasks/abc217_b

pub fn run(s: Vec<&str>) -> String {
    ["ABC", "ARC", "AGC", "AHC"]
        .iter()
        .find(|str| {
            !s.contains(str)
        })
        .unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(vec!["ARC", "AGC", "AHC"]));
        assert_eq!(String::from("AHC"), run(vec!["AGC", "ABC", "ARC"]));
    }
}
