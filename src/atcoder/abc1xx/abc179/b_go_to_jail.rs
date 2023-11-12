// https://atcoder.jp/contests/abc179/tasks/abc179_b

pub fn run(_n: usize, vec: Vec<(usize, usize)>) -> String {
    if vec.windows(3)
        .any(|d| {
            d.iter()
                .all(|(l, r)| {
                    l == r
                })
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
        assert_eq!(String::from("Yes"), run(5, vec![(1, 2), (6, 6), (4, 4), (3, 3), (3, 2)]));
        assert_eq!(String::from("No"), run(5, vec![(1, 1), (2, 2), (3, 4), (5, 5), (6, 6)]));
        assert_eq!(String::from("Yes"), run(6, vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]));
    }
}
