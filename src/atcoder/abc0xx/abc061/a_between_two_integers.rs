#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> String {
    if a <= c && c <= b {
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
        assert_eq!(String::from("Yes"), run(1, 3, 2));
        assert_eq!(String::from("No"), run(6, 5, 4));
        assert_eq!(String::from("Yes"), run(2, 2, 2));
    }
}
