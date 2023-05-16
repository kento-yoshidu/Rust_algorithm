pub fn run(k: u32, a: u32, b: u32) -> &'static str {
    for n in a..=b {
        println!("{}", n);
        if n % k == 0 {
            return "OK";
        }
    }

    "NG"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("OK", run(7, 500, 600));
        assert_eq!("NG", run(4, 5, 7));
        assert_eq!("OK", run(1, 11, 11));
    }
}