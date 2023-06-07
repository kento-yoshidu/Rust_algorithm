pub fn run(m: i32, h: i32) -> String {
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
