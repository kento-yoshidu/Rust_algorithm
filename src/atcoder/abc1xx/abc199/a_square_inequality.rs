// https://atcoder.jp/contests/abc199/tasks/abc199_a

pub fn run(a: usize, b: usize, c: usize) -> String {
    if c*c > a*a + b*b {
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
        assert_eq!(String::from("Yes"), run(2, 2, 4));
        assert_eq!(String::from("No"), run(10, 10, 10));
        assert_eq!(String::from("No"), run(3, 4, 5));
    }
}
