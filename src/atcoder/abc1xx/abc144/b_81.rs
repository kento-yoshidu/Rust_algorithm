// https://atcoder.jp/contests/abc144/tasks/abc144_b

pub fn run (n: usize) -> String {
    if n > 81 {
        return String::from("No");
    }

    for i in 0..=9 {
        for j in 0..=9 {
            if i * j == n {
                return String::from("Yes");
            }
        }
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(10));
        assert_eq!(String::from("No"), run(50));
        assert_eq!(String::from("Yes"), run(81));
    }
}
