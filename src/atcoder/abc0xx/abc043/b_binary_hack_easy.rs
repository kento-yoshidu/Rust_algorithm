// https://atcoder.jp/contests/abc043/tasks/abc043_b

pub fn run(s: String) -> String {
    let mut ans = Vec::<char>::new();

    for c in s.chars() {
        if c == '0' {
            ans.push('0');
        } else if c == '1' {
            ans.push('1');
        } else {
            ans.pop();
        }
    }

    ans.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("00"), run(String::from("01B0")));
        assert_eq!(String::from("1"), run(String::from("0BB1")));
    }
}
