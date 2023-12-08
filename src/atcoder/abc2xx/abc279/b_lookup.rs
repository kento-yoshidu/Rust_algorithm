// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run(s: &str, t: &str) -> String {
    if s.contains(t) {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run("voltage", "tag"));
        assert_eq!(String::from("No"), run("atcoder", "ace"));
        assert_eq!(String::from("No"), run("gorilla", "gorillagorillagorilla"));
        assert_eq!(String::from("Yes"), run("toyotasystems", "toyotasystems"));
    }
}
