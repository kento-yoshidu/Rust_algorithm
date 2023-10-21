// https://atcoder.jp/contests/abc011/tasks/abc011_2

pub fn run(s: String) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                (c.to_uppercase()).to_string()
            } else {
                (c.to_lowercase()).to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Takahashi"), run(String::from("taKahAshI")));
        assert_eq!(String::from("A"), run(String::from("A")));
    }
}
