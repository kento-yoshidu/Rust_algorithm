// https://atcoder.jp/contests/abc229/tasks/abc229_a

pub fn run(s1: &str, s2: &str) -> String {
    let s1_count = s1.chars().filter(|c| *c == '#').count();
    let s2_count = s2.chars().filter(|c| *c == '#').count();

    if s1_count + s2_count >= 3 {
        return String::from("Yes");
    }

    if s1_count == 2 || s2_count == 2 {
        return String::from("Yes");
    }

    if s1.chars().zip(s2.chars()).any(|v| {
        v.0 == '#' && v.1 == '#'
    }) {
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
        assert_eq!(String::from("Yes"), run("##", "#."));
        assert_eq!(String::from("No"), run(".#", "#."));
        assert_eq!(String::from("Yes"), run("##", ".."));
    }
}
