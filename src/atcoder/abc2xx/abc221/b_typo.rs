// https://atcoder.jp/contests/abc221/tasks/abc221_b

pub fn run(s: &str, t: &str) -> String {
    if s == t {
        return String::from("Yes");
    }

    for i in 0..t.len()-1 {
        let mut chars: Vec<char> = t.chars().collect();

        chars.swap(i, i+1);

        if s.chars().collect::<Vec<char>>() == chars {
            return String::from("Yes");
        }
    }

    String::from("No")
}

pub fn run2(s: &str, t: &str) -> String {
    if s == t {
        return String::from("Yes");
    }

    if (0..t.len()-1)
        .any(|i| {
            let mut chars: Vec<char> = t.chars().collect();

            chars.swap(i, i+1);

            s.chars().collect::<Vec<char>>() == chars
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
        assert_eq!(String::from("Yes"), run("abc", "acb"));
        assert_eq!(String::from("No"), run("aabb", "bbaa"));
        assert_eq!(String::from("Yes"), run("abcde", "abcde"));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2("abc", "acb"));
        assert_eq!(String::from("No"), run2("aabb", "bbaa"));
        assert_eq!(String::from("Yes"), run2("abcde", "abcde"));
    }
}
