pub fn run(s: String) -> String {
    let mut l1: i32;
    let mut kome :i32;
    let mut l2: i32;

    let l = s.find('|').unwrap();
    let r = s.rfind('|').unwrap();
    let o = s.find('*').unwrap();

    if l < o && o < r {
        String::from("in")
    } else {
        String::from("out")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("in"), run(String::from(".|..*...|.")));
        assert_eq!(String::from("out"), run(String::from(".|..|.*...")));
        assert_eq!(String::from("in"), run(String::from("|*|")));
    }
}