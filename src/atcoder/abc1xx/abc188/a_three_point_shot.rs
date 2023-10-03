// https://atcoder.jp/contests/abc188/tasks/abc188_a

pub fn run(x: isize, y: isize) -> String {
    if (x - y).abs() <= 2 {
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
        assert_eq!(String::from("Yes"), run(3, 5));
        assert_eq!(String::from("No"), run(16, 2));
        assert_eq!(String::from("No"), run(12, 15));
    }
}
