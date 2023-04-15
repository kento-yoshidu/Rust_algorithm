pub fn run(r: u16, g: u16, b: u16) -> String {
    let temp = r * 100 + g * 10 + b;

    if temp % 4 == 0 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(4, 3, 2));
        assert_eq!(String::from("No"), run(2, 3, 4));
    }
}
