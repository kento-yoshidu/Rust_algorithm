// https://atcoder.jp/contests/abc333/tasks/abc333_b

pub fn run(s: &str, t: &str) -> String {
    let vec = ["AB", "BA", "BC", "CB", "CD", "DC", "DE", "ED", "EA", "AE"];

    if vec.contains(&s) == vec.contains(&t) {
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
        assert_eq!(String::from("Yes"), run("AC", "EC"));
        assert_eq!(String::from("No"), run("DA", "EA"));
        assert_eq!(String::from("Yes"), run("BD", "BD"));
    }
}
