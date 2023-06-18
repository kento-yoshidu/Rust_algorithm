#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> String {
    let mut vec = vec![a, b, c];

    vec.sort();

    if vec[0] + vec[1] == vec[2] {
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
        assert_eq!(String::from("Yes"), run(10, 30, 20));
        assert_eq!(String::from("No"), run(30, 30, 100));
        assert_eq!(String::from("Yes"), run(56, 25, 31));
    }
}
