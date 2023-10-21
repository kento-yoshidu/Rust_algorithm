// https://atcoder.jp/contests/abc069/tasks/abc069_b

pub fn run(s: String) -> String {
    format!("{}{}{}", &s.chars().nth(0).unwrap(), s.len()-2, &s.chars().last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("i18n"), run(String::from("internationalization")));
        assert_eq!(String::from("s4s"), run(String::from("smiles")));
        assert_eq!(String::from("x1z"), run(String::from("xyz")));
    }
}
