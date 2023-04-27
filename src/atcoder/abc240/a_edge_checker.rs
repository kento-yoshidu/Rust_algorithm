pub fn run(a: i8, b: i8) -> String {
    if (a - b).abs() == 1 || (a - b).abs() == 9 {
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
        assert_eq!(String::from("Yes"), run(2, 3));
        assert_eq!(String::from("No"), run(3, 5));
        assert_eq!(String::from("Yes"), run(1, 10));
    }
}
