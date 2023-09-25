// https://atcoder.jp/contests/abc035/tasks/abc035_a

pub fn run(h: usize, w: usize) -> String {
    if h / 4 == w / 3 {
        String::from("4:3")
    } else {
        String::from("16:9")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("4:3"), run(4, 3));
        assert_eq!(String::from("16:9"), run(16, 9));
        assert_eq!(String::from("4:3"), run(28, 21));
    }
}
