// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_b

pub fn run(n: usize) -> String {
    (1..=n)
        .find(|i| {
            (*i as f64 * 1.08).floor() as usize == n
        })
        .map(|i| {
            i.to_string()
        })
        .unwrap_or(":(".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("400"), run(432));
        assert_eq!(String::from(":("), run(1079));
        assert_eq!(String::from("927"), run(1001));
    }
}
