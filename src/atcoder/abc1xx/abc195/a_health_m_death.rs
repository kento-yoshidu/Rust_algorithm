// https://atcoder.jp/contests/abc194/tasks/abc194_a

pub fn run(m: usize, h: usize) -> String {
    if h % m == 0 {
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
        assert_eq!(String::from("Yes"), run(10, 120));
        assert_eq!(String::from("No"), run(10, 125));
    }
}
