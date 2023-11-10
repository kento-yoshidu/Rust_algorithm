// https://atcoder.jp/contests/abc126/tasks/abc126_b

pub fn run(s: usize) -> String {
    let left = s / 100;
    let right = s % 100;

    // 月（01 ~ 12）であるか
    let is_left = 1 <= left && left <= 12;
    let is_right = 1 <= right && right <= 12;

    if is_left && is_right {
        String::from("AMBIGUOUS")
    } else if is_left {
        String::from("MMYY")
    } else if is_right {
        String::from("YYMM")
    } else {
        String::from("NA")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("YYMM"), run(1905));
        assert_eq!(String::from("MMYY"), run(0519));
        assert_eq!(String::from("AMBIGUOUS"), run(0112));
        assert_eq!(String::from("NA"), run(1700));
    }
}
