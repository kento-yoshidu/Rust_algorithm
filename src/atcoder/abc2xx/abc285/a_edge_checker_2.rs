// https://atcoder.jp/contests/abc285/tasks/abc285_a

pub fn run(a: usize, b: usize) -> String {
    if a*2 == b || a*2+1 == b {
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
        assert_eq!(String::from("Yes"), run(1, 2));
        assert_eq!(String::from("No"), run(2, 8));
        assert_eq!(String::from("No"), run(14, 15));
    }
}
