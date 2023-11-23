// https://atcoder.jp/contests/abc174/tasks/abc174_a

pub fn run(x: usize) -> String {
    if x >= 30 {
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
        assert_eq!(String::from("Yes"), run(30));
        assert_eq!(String::from("No"), run(25));
    }
}
