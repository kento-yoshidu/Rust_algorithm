// https://atcoder.jp/contests/abc063/tasks/abc063_a

pub fn run(a: i32, b: i32) -> String {
    if a + b >= 10 {
        String::from("error")
    } else {
        (a + b).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("9"), run(6, 3));
        assert_eq!(String::from("error"), run(6, 4));
    }
}
