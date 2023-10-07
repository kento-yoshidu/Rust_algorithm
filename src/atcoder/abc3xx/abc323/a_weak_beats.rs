// https://atcoder.jp/contests/abc323/tasks/abc323_a

pub fn run(s: String) -> String {
    if s.chars()
        .skip(1)
        .step_by(2)
        .all(|c| { c == '0' })
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
        assert_eq!(String::from("No"), run(String::from("1001000000001010")));
        assert_eq!(String::from("Yes"), run(String::from("1010100000101000")));
        assert_eq!(String::from("No"), run(String::from("1111111111111111")));
    }
}
