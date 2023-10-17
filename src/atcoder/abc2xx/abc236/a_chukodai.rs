// https://atcoder.jp/contests/abc236/tasks/abc236_a

pub fn run(s: String, a: usize, b: usize) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.swap(a-1, b-1);

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("chukodai"), run(String::from("chokudai"), 3, 5));
        assert_eq!(String::from("aa"), run(String::from("aa"), 1, 2));
        assert_eq!(String::from("baaabbba"), run(String::from("aaaabbbb"), 1, 8));
    }
}
