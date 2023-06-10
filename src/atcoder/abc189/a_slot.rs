pub fn run(s: String) -> String {
    let mut c = s.chars();

    let l = c.next().unwrap();
    let m = c.next().unwrap();
    let r = c.next().unwrap();

    if l == m && m == r {
        String::from("Won")
    } else {
        String::from("Lost")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Won"), run(String::from("SSS")));
        assert_eq!(String::from("Lost"), run(String::from("WVW")));
    }
}
