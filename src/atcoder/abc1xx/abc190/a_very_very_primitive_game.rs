// https://atcoder.jp/contests/abc190/tasks/abc190_a

#[allow(dead_code)]
pub fn run(t: i32, a: i32, n: i32) -> String {
    if t == a {
        if n == 0 {
            return String::from("Aoki");
        } else {
            return String::from("Takahashi");
        }
    }

    if t > a {
        String::from("Takahashi")
    } else {
        String::from("Aoki")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Takahashi"), run(2, 1, 0));
        assert_eq!(String::from("Aoki"), run(2, 2, 0));
        assert_eq!(String::from("Takahashi"), run(2, 2, 1));
    }
}
