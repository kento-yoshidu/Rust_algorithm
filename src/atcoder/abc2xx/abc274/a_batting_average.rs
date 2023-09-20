// https://atcoder.jp/contests/abc274/tasks/abc274_a

pub fn run(a: f64, b: f64) -> String {
    format!("{:.3}", b / a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("0.571"), run(7.0, 4.0));
        assert_eq!(String::from("0.429"), run(7.0, 3.0));
        assert_eq!(String::from("0.500"), run(2.0, 1.0));
        assert_eq!(String::from("1.000"), run(10.0, 10.0));
        assert_eq!(String::from("0.000"), run(1.0, 0.0));
    }
}
