// https://atcoder.jp/contests/abc312/tasks/abc312_a

pub fn run(s: &str) -> String {
    if ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"].contains(&s) {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("No"), run("ABC"));
        assert_eq!(String::from("Yes"), run("FAC"));
        assert_eq!(String::from("No"), run("XYX"));
    }
}
