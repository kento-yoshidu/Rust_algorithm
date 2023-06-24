#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("CDEZAB"), run(2, String::from("ABCXYZ")));
        assert_eq!(String::from("ABCXYZ"), run(0, String::from("ABCXYZ")));
        assert_eq!(String::from("NOPQRSTUVWXYZABCDEFGHIJKLM"), run(13, String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")));
    }
}
