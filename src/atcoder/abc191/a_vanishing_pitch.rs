pub fn run(v: i32, t: i32, s: i32, d: i32) -> String {
    if d < v*t || v*s < d {
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
        assert_eq!(String::from("Yes"), run(10, 3, 5, 20));
        assert_eq!(String::from("No"), run(10, 3, 5, 30));
    }
}
