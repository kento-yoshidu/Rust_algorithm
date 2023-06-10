#[allow(dead_code)]
pub fn run(x: i32, y: i32) -> String {
    let abs = (x - y).abs();

    if abs <= 2 {
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
