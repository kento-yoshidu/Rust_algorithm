// https://atcoder.jp/contests/abc190/tasks/abc190_b

pub fn run(_n: usize, s: usize, d: usize, v: Vec<(usize, usize)>) -> String {
    if v.iter()
        .any(|(x, y)| {
            *x < s && *y > d
        }) {
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
        assert_eq!(String::from("Yes"), run(4, 9, 9, vec![(5, 5), (15, 5), (5, 15), (15, 15)]));
        assert_eq!(String::from("No"), run(3, 691, 273, vec![(691, 997), (593, 273), (691, 273)]));
        assert_eq!(String::from("Yes"), run(7, 100, 100, vec![(10, 11), (12, 67), (192, 79), (154, 197), (142, 158), (20, 25), (17, 108)]));
    }
}
