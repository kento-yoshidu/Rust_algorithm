// https://atcoder.jp/contests/abc264/tasks/abc264_a

pub fn run(l: usize, r: usize) -> String {
    "atcoder"[l-1..=r-1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("code"), run(3, 6));
        assert_eq!(String::from("o"), run(4, 4));
        assert_eq!(String::from("atcoder"), run(1, 7));
    }
}
