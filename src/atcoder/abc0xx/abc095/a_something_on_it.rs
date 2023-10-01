// https://atcoder.jp/contests/abc095/tasks/abc095_a

pub fn run(s: String) -> usize {
    700 + 100 * s.chars().filter(|c| { *c == 'o' }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(900, run(String::from("oxo")));
        assert_eq!(1000, run(String::from("ooo")));
    }
}
