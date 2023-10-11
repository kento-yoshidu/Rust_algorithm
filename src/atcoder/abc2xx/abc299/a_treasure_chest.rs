// https://atcoder.jp/contests/abc299/tasks/abc299_a

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

pub fn run2(_n: usize, s: String) -> String {
    let mut l = -1;
    let mut m = -1;
    let mut r = -1;

    for (i, c) in s.chars().enumerate() {
        if c == '|' {
            if l == -1 {
                l = i as i32;
            } else {
                r = i as i32;
            }
        }

        if c == '*' {
            m = i as i32;
        }
    }

    if l < m && m < r {
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

    #[test]
    fn test2() {
        assert_eq!(String::from("in"), run2(10, String::from(".|..*...|.")));
        assert_eq!(String::from("out"), run2(10, String::from(".|..|.*...")));
        assert_eq!(String::from("in"), run2(3, String::from("|*|")));
    }
}
