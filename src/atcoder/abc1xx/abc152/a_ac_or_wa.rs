pub fn run(n: usize, m: usize) -> String {
    if n == m {
        String::from("AC")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("AC"), run(3, 3));
        assert_eq!(String::from("No"), run(3, 2));
        assert_eq!(String::from("AC"), run(1, 1));
    }
}

