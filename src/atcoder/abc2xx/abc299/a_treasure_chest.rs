#[allow(dead_code)]
pub fn run(_n: usize, s: String) -> String {
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
        assert_eq!(String::from("in"), run(10, String::from(".|..*...|.")));
        assert_eq!(String::from("out"), run(10, String::from(".|..|.*...")));
        assert_eq!(String::from("in"), run(3, String::from("|*|")));
    }
}
