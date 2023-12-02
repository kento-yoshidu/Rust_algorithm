// https://atcoder.jp/contests/abc146/tasks/abc146_a

pub fn run(s: &str) -> usize {
    let arr = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

    arr.iter()
        .rev()
        .position(|day| s == *day)
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run("SAT"));
        assert_eq!(2, run("FRI"));
        assert_eq!(3, run("THU"));
        assert_eq!(4, run("WED"));
        assert_eq!(5, run("TUE"));
        assert_eq!(6, run("MON"));
        assert_eq!(7, run("SUN"));
    }
}
