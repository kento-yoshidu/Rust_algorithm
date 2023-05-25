pub fn run(price: u16, coin: u16) -> String {
    let rest = price % 500;

    if rest <= coin {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(2018, 218));
        assert_eq!(String::from("No"), run(2763, 0));
        assert_eq!(String::from("Yes"), run(37, 514));
        assert_eq!(String::from("Yes"), run(37, 37));
        assert_eq!(String::from("No"), run(37, 36));
    }
}
