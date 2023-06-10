#[allow(dead_code)]
pub fn run(n: i32) -> i32 {
    if n % 100 == 0 {
        n / 100
    } else {
        n / 100 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(21, run(2021));
        assert_eq!(2, run(200));
    }
}
