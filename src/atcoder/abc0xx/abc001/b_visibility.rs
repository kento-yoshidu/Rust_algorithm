// https://atcoder.jp/contests/abc001/tasks/abc001_2

pub fn run(m: usize) -> String {
    if m < 100 {
        String::from("00")
    } else if 100 <= m && m <= 5000 {
        format!("{:02}", m / 100)
    } else if 6000 <= m && m <= 30000 {
        format!("{}", m / 1000 + 50)
    } else if 35000 <= m && m <= 70000 {
        format!("{}", ((m / 1000) - 30) / 5 + 80)
    } else {
        String::from("89")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("65"), run(15000));
        assert_eq!(String::from("89"), run(75000));
        assert_eq!(String::from("02"), run(200));
        assert_eq!(String::from("20"), run(2000));
        assert_eq!(String::from("82"), run(40000));
    }
}
