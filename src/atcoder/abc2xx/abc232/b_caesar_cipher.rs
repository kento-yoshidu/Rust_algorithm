// https://atcoder.jp/contests/abc232/tasks/abc232_b

fn next_char(c: char, n: u8) -> char {
    let val = (c as u8 - 97 + n) % 26 + 97;
    val as char
}

fn run(s: &str, t: &str) -> String {
    if (0..26)
        .any(|i| {
            let str: String = s.chars().map(|c| next_char(c, i)).collect();

            str == t
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run("abc", "ijk"));
        assert_eq!(String::from("Yes"), run("z", "a"));
        assert_eq!(String::from("No"), run("ppq", "qqp"));
        assert_eq!(String::from("Yes"), run("atcoder", "atcoder"));
    }
}
