// https://atcoder.jp/contests/abc202/tasks/abc202_b

pub fn run(s: String) -> String {
    s.chars()
        .rev()
        .map(|c| {
            match c {
                '6' => '9',
                '9' => '6',
                _ => c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("6881090"), run(String::from("0601889")));
        assert_eq!(String::from("01698"), run(String::from("86910")));
        assert_eq!(String::from("01010"), run(String::from("01010")));
    }
}
