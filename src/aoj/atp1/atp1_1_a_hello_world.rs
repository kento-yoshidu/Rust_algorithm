#[allow(dead_code)]
pub fn run() -> String {
    String::from("Hello World")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Hello World"), run());
    }
}
