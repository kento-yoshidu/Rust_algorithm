// https://atcoder.jp/contests/abc327/tasks/abc327_a

pub fn run(_n: usize, s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    if chars.windows(2)
        .any(|t| {
            t[0] == 'a' && t[1] == 'b' || t[0] == 'b' && t[1] == 'a'
            })
        {
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
        assert_eq!(String::from("Yes"), run(3, String::from("abc")));
        assert_eq!(String::from("Yes"), run(3, String::from("ba")));
        assert_eq!(String::from("No"), run(7, String::from("atcoder")));
    }
}
