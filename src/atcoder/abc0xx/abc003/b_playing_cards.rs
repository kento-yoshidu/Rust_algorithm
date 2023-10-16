// https://atcoder.jp/contests/abc003/tasks/abc003_2

pub fn run(s: String, t: String) -> String {
    let arr = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

    if s.chars().zip(t.chars()).all(|t| {
        if t.0 == t.1 {
            true
        } else {
            if t.0 == '@' && t.1 == '@' {
                true
            } else if t.0 != '@' && t.1 != '@' {
                false
            } else if t.0 == '@' && arr.contains(&t.1) {
                true
            } else if t.1 == '@' && arr.contains(&t.0) {
                true
            } else {
                false
            }
        }
    }) {
        String::from("You can win")
    } else {
        String::from("You will lose")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("You can win"), run(String::from("ch@ku@ai"), String::from("choku@@i")));
        assert_eq!(String::from("You will lose"), run(String::from("aoki"), String::from("@ok@")));
        assert_eq!(String::from("You will lose"), run(String::from("arc"), String::from("abc")));
    }
}