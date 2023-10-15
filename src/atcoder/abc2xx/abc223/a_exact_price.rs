// https://atcoder.jp/contests/abc223/tasks/abc223_a

pub fn run(x: usize) -> String {
    if x % 100 == 0 && x != 0 {
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
        assert_eq!(String::from("Yes"), run(500));
        assert_eq!(String::from("No"), run(40));
        assert_eq!(String::from("No"), run(0));
    }
}
