#[allow(dead_code)]
pub fn run(n: Vec<i32>) -> String {
    if n[0] == 1 {
        String::from("Hello World")
    } else {
        (n[1] + n[2]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Hello World"), run(vec![1]));
        assert_eq!(String::from("8"), run(vec![2, 3, 5]));
    }
}
