#[allow(dead_code)]
pub fn run(n: usize, vec: &mut Vec<(&str, i32)>) -> String {
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    vec[n - 2].0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("K2"), run(3, &mut vec![("Everest", 8849), ("K2", 8611), ("Kangchenjunga", 8586)]));
        assert_eq!(String::from("Kita"), run(4, &mut vec![("Kita", 3193), ("Aino", 3189), ("Fuji", 3776), ("Okuhotaka", 3190)]));
        assert_eq!(String::from("QCFium"), run(4, &mut vec![("QCFium", 2846), ("chokudai", 2992), ("kyoprofriends", 2432), ("penguinman", 2390)]));
    }
}
