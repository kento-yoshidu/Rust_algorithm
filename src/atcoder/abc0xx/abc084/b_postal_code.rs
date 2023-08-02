// https://atcoder.jp/contests/abc084/tasks/abc084_b

#[allow(dead_code)]
pub fn run(a: usize, _b: usize, s: String) -> String {
    let c: Vec<char> = s.chars().collect();

    if c[a] != '-' {
        return String::from("No")
    }

    if c.iter().filter(|&c| {
        *c == '-'
    }).count() != 1 {
        return String::from("No");
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(3, 4, String::from("269-6650")));
        assert_eq!(String::from("No"), run(1, 1, String::from("---")));
        assert_eq!(String::from("No"), run(1, 2, String::from("7444")));
    }
}
