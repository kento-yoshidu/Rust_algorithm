#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> String {
    if a * 6 >= b {
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
        assert_eq!(String::from("Yes"), run(2, 11));
        assert_eq!(String::from("No"), run(2, 13));
        assert_eq!(String::from("Yes"), run(100, 600));
    }
}
