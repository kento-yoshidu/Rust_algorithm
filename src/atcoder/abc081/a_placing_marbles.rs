pub fn run(str: &str) -> i32 {
    let mut count = 0;

    for c in str.chars() {
        if c == '1' {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run("101"));
        assert_eq!(0, run("000"));
    }
}