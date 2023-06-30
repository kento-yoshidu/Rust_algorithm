// https://atcoder.jp/contests/abc060/tasks/abc060_a

#[allow(dead_code)]
fn run(a: String, b: String, c: String) -> String {
    let ar = a.chars().last().unwrap();
    let bl = b.chars().nth(0).unwrap();
    let br = b.chars().last().unwrap();
    let cl = c.chars().nth(0).unwrap();

    if ar == bl && br == cl {
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
        assert_eq!(String::from("Yes"), run(String::from("rng"), String::from("gorilla"), String::from("apple")));
        assert_eq!(String::from("No"), run(String::from("yakiniku"), String::from("unagi"), String::from("sushi")));
        assert_eq!(String::from("Yes"), run(String::from("a"), String::from("a"), String::from("a")));
        assert_eq!(String::from("No"), run(String::from("aaaaaaaaab"), String::from("aaaaaaaaaa aaaaaaaaab"), String::from("a")));
    }
}
