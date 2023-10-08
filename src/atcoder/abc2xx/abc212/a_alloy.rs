// https://atcoder.jp/contests/abc212/tasks/abc212_a

pub fn run(a: usize, b: usize) -> String {
    if a > 0 && b > 0 {
        String::from("Alloy")
    } else if b == 0 {
        String::from("Gold")
    } else {
        String::from("Silver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Alloy"), run(50, 50));
        assert_eq!(String::from("Gold"), run(100, 0));
        assert_eq!(String::from("Silver"), run(0, 100));
        assert_eq!(String::from("Alloy"), run(100, 2));
    }
}
