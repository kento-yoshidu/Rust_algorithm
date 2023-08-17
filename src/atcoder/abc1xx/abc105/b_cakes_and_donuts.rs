// https://atcoder.jp/contests/abc105/tasks/abc105_b

pub fn run(n: usize) -> String {
    for i in 0..=n/4 {
        for j in 0..=n/7 {
            if i*4 + j*7 == n {
                return String::from("Yes");
            }
        }

    }

    return String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(11));
        assert_eq!(String::from("Yes"), run(40));
        assert_eq!(String::from("No"), run(3));
        assert_eq!(String::from("No"), run(13));
        assert_eq!(String::from("Yes"), run(41));
    }
}
