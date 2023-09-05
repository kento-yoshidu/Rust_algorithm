// https://atcoder.jp/contests/abc104/tasks/abc104_a

pub fn run(r: usize) -> String {
    if r < 1200 {
        String::from("ABC")
    } else if r < 2800 {
        String::from("ARC")
    } else {
        String::from("AGC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(1199));
        assert_eq!(String::from("ARC"), run(1200));
        assert_eq!(String::from("AGC"), run(4208));
    }
}
