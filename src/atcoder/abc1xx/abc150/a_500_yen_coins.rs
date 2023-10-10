// https://atcoder.jp/contests/abc150/tasks/abc150_a

pub fn run(k: usize, x: usize) -> String {
    if k * 500 >= x {
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
        assert_eq!(String::from("Yes"), run(2, 900));
        assert_eq!(String::from("No"), run(1, 501));
        assert_eq!(String::from("Yes"), run(4, 2000));
    }
}
