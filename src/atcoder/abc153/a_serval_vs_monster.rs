pub fn run(hp: i32, a: i32) -> i32 {
    if hp % a == 0 {
        hp / a
    } else {
        hp / a + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(10, 4));
        assert_eq!(10000, run(10000, 1));
        assert_eq!(1, run(1, 10000));
    }
}
