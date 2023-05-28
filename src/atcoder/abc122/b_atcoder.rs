pub fn run(str: String) -> i32 {
    let mut len = 0;
    let mut max_len = 0;

    for c in str.chars() {
        if "ACGT".contains(c) {
            len += 1;
            max_len = max_len.max(len);
        } else {
            len = 0;
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(String::from("ATCODER")));
        assert_eq!(5, run(String::from("HATAGAYA")));
        assert_eq!(0, run(String::from("SHINJUKU")));
    }
}
