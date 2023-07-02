#[allow(dead_code)]
pub fn run(s: String) -> String {
    let c = s.chars().last().unwrap();

    match c {
        's' => s + "es",
        _ => s + "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("apples"), run(String::from("apple")));
        assert_eq!(String::from("buses"), run(String::from("bus")));
        assert_eq!(String::from("boxs"), run(String::from("box")));
    }
}
