// https://atcoder.jp/contests/abc059/tasks/abc059_b

pub fn run(a: &str, b: &str) -> String {
    if a.len() > b.len() {
        String::from("GREATER")
    } else if a.len() < b.len() {
        String::from("LESS")
    } else if a == b {
        String::from("EQUAL")
    } else if a > b {
        String::from("GREATER")
    } else {
        String::from("LESS")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("GREATER"), run("36", "24"));
        assert_eq!(String::from("LESS"), run("850", "3777"));
        assert_eq!(String::from("LESS"), run("9720246", "22516266"));
        assert_eq!(String::from("LESS"), run("123456789012345678901234567890", "234567890123456789012345678901"));
    }
}
