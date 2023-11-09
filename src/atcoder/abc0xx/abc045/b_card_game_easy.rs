// https://atcoder.jp/contests/abc045/tasks/abc045_b

// https://atcoder.jp/contests/abc045/tasks/abc045_b

pub fn run(a: &str, b: &str, c: &str) -> String {
    let mut vec_a = a.chars().collect::<Vec<char>>();
    let mut vec_b = b.chars().collect::<Vec<char>>();
    let mut vec_c = c.chars().collect::<Vec<char>>();

    let mut target = vec_a[0];

    loop {
        if target == 'a' {
            if vec_a.len() == 0 {
                return "A".to_string();
            } else {
                target = *vec_a.iter().nth(0).unwrap();
                vec_a.remove(0);
            }
        } else if target == 'b' {
            if vec_b.len() == 0 {
                return "B".to_string()
            } else {
                target = *vec_b.iter().nth(0).unwrap();
                vec_b.remove(0);
            }
        } else {
            if vec_c.len() == 0 {
                return "C".to_string()
            } else {
                target = *vec_c.iter().nth(0).unwrap();
                vec_c.remove(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('A'.to_string(), run("aca", "accc", "ca"));
        assert_eq!('C'.to_string(), run("abcb", "aacb", "bccc"));
        assert_eq!('A'.to_string(), run("a", "a", "a"));
        assert_eq!('B'.to_string(), run("b", "b", "b"));
    }
}
