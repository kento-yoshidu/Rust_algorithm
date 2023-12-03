// https://atcoder.jp/contests/abc331/tasks/abc331_a

pub fn run(m_: usize, d_: usize, y: usize, m: usize, d: usize) -> String {
    if m_ == m && d_ == d {
        return format!("{} 1 1", y + 1);
    } else if d_ == d {
        return format!("{} {} {}", y, m + 1, 1);
    } else {
        format!("{} {} {}", y, m , d+1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("2024 1 1"), run(12, 30, 2023, 12, 30));
        assert_eq!(String::from("6789 23 46"), run(36, 72, 6789, 23, 45));
        assert_eq!(String::from("2012 6 21"), run(12, 30, 2012, 6, 20));
        assert_eq!(String::from("2012 7 1"), run(12, 30, 2012, 6, 30));
    }
}
