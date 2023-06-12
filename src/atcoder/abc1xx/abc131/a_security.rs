#[allow(dead_code)]
pub fn run(num: i32) -> String {
    let a = num / 1000;
    let b = num / 100 % 10;
    let c = num / 10 % 10;
    let d = num % 10;

    if a == b || b == c || c == d {
        String::from("Bad")
    } else {
        String::from("Good")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Bad"), run(3776));
        assert_eq!(String::from("Good"), run(8080));
        assert_eq!(String::from("Bad"), run(1333));
        assert_eq!(String::from("Bad"), run(0024));
    }
}
