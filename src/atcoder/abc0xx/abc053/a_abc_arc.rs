// https://atcoder.jp/contests/abc053/tasks/abc053_a

pub fn run(x: i32) -> String {
    if x < 1200 {
        String::from("ABC")
    } else {
        String::from("ARC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(1000));
        assert_eq!(String::from("ARC"), run(2000));
    }
}
