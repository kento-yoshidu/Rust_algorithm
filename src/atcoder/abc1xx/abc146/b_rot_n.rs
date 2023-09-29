// https://atcoder.jp/contests/abc146/tasks/abc146_b

pub fn run(n: u8, str: String) -> String {
    let mut result = String::from("");

    for c in str.chars() {
        let mut tmp = c as u8 + n;

        if tmp as u8 > 90 {
            tmp -= 26;
        }

        result.push(tmp as char);
    }

    result
}

pub fn run2(n: u8, str: String) -> String {
    str
        .chars()
        .map(|c| {
            c as u8 + n
        })
        .map(|c| {
            if c > 90 {
                (c - 26) as char
            } else {
                c as char
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("CDEZAB"), run(2, String::from("ABCXYZ")));
        assert_eq!(String::from("ABCXYZ"), run(0, String::from("ABCXYZ")));
        assert_eq!(String::from("NOPQRSTUVWXYZABCDEFGHIJKLM"), run(13, String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("CDEZAB"), run2(2, String::from("ABCXYZ")));
        assert_eq!(String::from("ABCXYZ"), run2(0, String::from("ABCXYZ")));
        assert_eq!(String::from("NOPQRSTUVWXYZABCDEFGHIJKLM"), run2(13, String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")));
    }
}
