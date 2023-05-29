pub fn run(a: i32, b: i32) -> &'static str {
    if (a * b) % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Even", run(3, 4));
        assert_eq!("Odd", run(3, 3));
    }
}
