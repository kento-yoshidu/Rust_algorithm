// https://atcoder.jp/contests/abc153/tasks/abc153_b

pub fn run(h: usize, _n: usize, a: Vec<usize>) -> String {
    if h <= a.iter().sum() {
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
        assert_eq!(String::from("Yes"), run(10, 3, vec![4, 5, 6]));
        assert_eq!(String::from("No"), run(20, 3, vec![4, 5, 6]));
        assert_eq!(String::from("Yes"), run(210, 5, vec![31, 41, 59, 26, 53]));
        assert_eq!(String::from("No"), run(211, 5, vec![31, 41, 59, 26, 53]));
    }
}
