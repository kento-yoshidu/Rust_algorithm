// https://atcoder.jp/contests/abc131/tasks/abc131_a

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

pub fn run2(s: String) -> String {
    let vec: Vec<char> = s.chars().collect();

    if vec.windows(2).any(|v| { v[0] == v[1] }) {
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

    #[test]
    fn test2() {
        assert_eq!(String::from("Bad"), run2(String::from("3776")));
        assert_eq!(String::from("Good"), run2(String::from("8080")));
        assert_eq!(String::from("Bad"), run2(String::from("1333")));
        assert_eq!(String::from("Bad"), run2(String::from("0024")));
    }
}
