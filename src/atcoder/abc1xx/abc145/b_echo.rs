// https://atcoder.jp/contests/abc145/tasks/abc145_b

pub fn run(n: usize, s: &str) -> String {
    if n % 2 != 0 {
        return String::from("No");
    }

    if &s[0..(n/2)] == &s[(n/2)..] {
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
        assert_eq!(String::from("Yes"), run(6, "abcabc"));
        assert_eq!(String::from("No"), run(6, "abcadc"));
        assert_eq!(String::from("No"), run(1, "z"));
    }
}
