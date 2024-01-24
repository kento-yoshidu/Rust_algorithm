// https://atcoder.jp/contests/abc233/tasks/abc233_b

pub fn run(l: usize, r: usize, s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars[l-1..=r-1].reverse();

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("abgfedch"), run(3, 7, "abcdefgh"));
        assert_eq!(String::from("reviver"), run(1, 7, "reviver"));
        assert_eq!(String::from("meramtsirhcyrs"), run(4, 13, "merrychristmas"));
    }
}
