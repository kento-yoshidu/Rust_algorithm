// https://atcoder.jp/contests/abc101/tasks/abc101_a

pub fn run(s: String) -> isize {
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
