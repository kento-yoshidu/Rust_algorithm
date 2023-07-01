#[allow(dead_code)]
pub fn run(n: u32) -> i32 {
    2_i32.pow(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(3));
        assert_eq!(1073741824, run(30));
    }
}
