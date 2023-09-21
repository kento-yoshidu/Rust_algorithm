// https://atcoder.jp/contests/abc065/tasks/abc065_a

pub fn run(x: i32, a: i32, b: i32) -> String {
    if a > b {
        return String::from("delicious");
    }

    let expire = b - a;

    if x >= expire {
        return String::from("safe");
    } else {
        return String::from("dangerous")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("safe"), run(4, 3, 6));
        assert_eq!(String::from("delicious"), run(6, 5, 1));
        assert_eq!(String::from("dangerous"), run(3, 7, 12));
    }
}
