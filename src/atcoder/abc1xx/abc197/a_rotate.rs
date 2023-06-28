#[allow(dead_code)]
pub fn run(s: String) -> String {
    let mut c = s.chars();

    let mut ans = String::new();

    let c1 = c.nth(0).unwrap();
    let c2 = c.nth(0).unwrap();
    let c3 = c.nth(0).unwrap();

    ans.push(c2);
    ans.push(c3);
    ans.push(c1);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("bca"), run(String::from("abc")));
        assert_eq!(String::from("aba"), run(String::from("aab")));
    }
}
