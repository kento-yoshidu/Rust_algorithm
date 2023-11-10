// https://atcoder.jp/contests/abc160/tasks/abc160_a

pub fn run(s: &str) -> String {
    let t = &s.chars().nth(2).unwrap();
    let fo = &s.chars().nth(3).unwrap();
    let fi = &s.chars().nth(4).unwrap();
    let s = &s.chars().nth(5).unwrap();

    if t == fo && fi == s {
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
        assert_eq!(String::from("Yes"), run("sippuu"));
        assert_eq!(String::from("No"), run("iphone"));
        assert_eq!(String::from("Yes"), run("coffee"));
    }
}
