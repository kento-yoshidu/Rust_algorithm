// https://atcoder.jp/contests/abc101/tasks/abc101_a

#[allow(dead_code)]
pub fn run(s: String) -> i32 {
    s.chars().map(|c| {
        if c == '+' {
            1
        } else {
            -1
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(String::from("+-++")));
        assert_eq!(-2, run(String::from("-+--")));
        assert_eq!(-4, run(String::from("----")));
    }
}
