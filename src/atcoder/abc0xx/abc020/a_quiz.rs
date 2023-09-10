// https://atcoder.jp/contests/abc020/tasks/abc020_a

pub fn run(q: usize) -> String {
    if q == 1 {
        String::from("ABC")
    } else {
        String::from("chokudai")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("ABC", run(1));
        assert_eq!("chokudai", run(2));
    }
}
