// https://atcoder.jp/contests/abc292/tasks/abc292_a

pub fn run(s: String) -> String {
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(String::from("abc")));
        assert_eq!(String::from("A"), run(String::from("a")));
        assert_eq!(String::from("ABCDEFGHJIKLNMOQPRSTVUWXYZ"), run(String::from("abcdefghjiklnmoqprstvuwxyz")));
    }
}
