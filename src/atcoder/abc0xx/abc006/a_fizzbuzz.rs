// https://atcoder.jp/contests/abc006/tasks/abc006_1

#[allow(dead_code)]
pub fn run(n: u8) -> String{
    if n % 3 == 0 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        assert_eq!(String::from("No"), run(2));
        assert_eq!(String::from("Yes"), run(9));
        assert_eq!(String::from("Yes"), run(3));
    }
}
