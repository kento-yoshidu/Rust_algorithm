pub fn run(str: &str) -> usize {
    for n in 0..9 {
        if n != str.chars().nth(n).unwrap() as usize - 48 {
            return n;
        }
    }

    9
}

#[cfg(test)]
mod tets {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run("012456789"));
        assert_eq!(0, run("123456789"));
        assert_eq!(9, run("012345678"));
    }
}
