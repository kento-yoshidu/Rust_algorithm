// https://atcoder.jp/contests/abc237/tasks/abc237_c

fn check(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

// Refactoring
pub fn run(s: String) -> String {
    let mut vec: Vec<char> = s.chars().collect();

    for c in vec.clone().into_iter().rev() {
        if c == 'a' {
            vec.pop().unwrap();
        } else {
            break
        }
    }

    let str: String = vec.iter().collect();

    if check(str) {
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
        assert_eq!(String::from("Yes"), run(String::from("kasaka")));
        assert_eq!(String::from("No"), run(String::from("atcoder")));
        assert_eq!(String::from("Yes"), run(String::from("php")));
    }
}
