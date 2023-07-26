// https://atcoder.jp/contests/abc054/tasks/abc054_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> String {
    if a == b {
        return String::from("Draw");
    }

    if a == 1 {
        return String::from("Alice");
    }

    if b == 1 {
        return String::from("Bob");
    }

    if a > b {
        return String::from("Alice");
    } else {
        return String::from("Bob");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Alice"), run(8, 6));
        assert_eq!(String::from("Draw"), run(1, 1));
        assert_eq!(String::from("Bob"), run(13, 1));
    }
}
