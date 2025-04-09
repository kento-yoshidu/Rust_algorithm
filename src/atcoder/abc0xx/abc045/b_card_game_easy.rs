// https://atcoder.jp/contests/abc045/tasks/abc045_b

fn run(a: &str, b: &str, c: &str) -> char {
    let mut vec_a = a.chars().collect::<Vec<char>>();
    let mut vec_b = b.chars().collect::<Vec<char>>();
    let mut vec_c = c.chars().collect::<Vec<char>>();

    let mut target = vec_a[0];

    loop {
        if target == 'a' {
            if vec_a.len() == 0 {
                return 'A';
            } else {
                target = *vec_a.iter().nth(0).unwrap();
                vec_a.remove(0);
            }
        } else if target == 'b' {
            if vec_b.len() == 0 {
                return 'B';
            } else {
                target = *vec_b.iter().nth(0).unwrap();
                vec_b.remove(0);
            }
        } else {
            if vec_c.len() == 0 {
                return 'C';
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

    struct TestCase(&'static str, &'static str, &'static str, char);

    #[test]
    fn test() {
        let tests = [
            TestCase("aca", "accc", "ca", 'A'),
            TestCase("abcb", "aacb", "bccc", 'C'),
            TestCase("a", "a", "a", 'A'),
            TestCase("b", "b", "b", 'B'),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
