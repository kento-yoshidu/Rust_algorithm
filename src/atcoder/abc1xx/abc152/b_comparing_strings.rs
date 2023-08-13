// https://atcoder.jp/contests/abc152/tasks/abc152_b

#[allow(dead_code)]
pub fn run(a: usize, b: usize) -> String {
    a.min(b).to_string().repeat(a.max(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("3333"), run(4, 3));
        assert_eq!(String::from("7777777"), run(7, 7));
    }
}
