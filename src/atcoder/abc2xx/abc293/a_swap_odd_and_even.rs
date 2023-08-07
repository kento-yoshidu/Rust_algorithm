// https://atcoder.jp/contests/abc293/tasks/abc293_a

pub fn run(s: String) -> String {
    let mut ans = String::new();

    let str: Vec<char> = s.chars().map(|c| c).collect();

    for i in (0..str.len()).step_by(2) {
        ans.push(str[i+1]);
        ans.push(str[i]);
    }

    ans
}

pub fn run2(s: String) -> String {
    let mut str: Vec<char> = s.chars().map(|c| c).collect();

    for i in (0..str.len()).step_by(2) {
        let tmp = str[i];

        str[i] = str[i+1];
        str[i+1] = tmp;
    }

    str.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("badcfe"), run(String::from("abcdef")));
        assert_eq!(String::from("aaaa"), run(String::from("aaaa")));
        assert_eq!(String::from("atcoderbeginnercontest"), run(String::from("taocedbrgeniencrnoetts")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("badcfe"), run2(String::from("abcdef")));
        assert_eq!(String::from("aaaa"), run2(String::from("aaaa")));
        assert_eq!(String::from("atcoderbeginnercontest"), run2(String::from("taocedbrgeniencrnoetts")));
    }
}
