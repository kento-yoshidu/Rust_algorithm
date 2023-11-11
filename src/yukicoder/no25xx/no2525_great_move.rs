// https://yukicoder.me/problems/no/2525

pub fn run(h: isize, s: isize) -> String {
    if (h + s) % 2 == 0 {
        String::from("Possible")
    } else {
        String::from("Impossible")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Possible"), run(-1, 1));
        assert_eq!(String::from("Possible"), run(-3, 1));
        assert_eq!(String::from("Impossible"), run(0, 1));
        assert_eq!(String::from("Possible"), run(998244353, 1000000007));
        assert_eq!(String::from("Possible"), run(98526, -24));
        assert_eq!(String::from("Impossible"), run(86, -56857));
        assert_eq!(String::from("Impossible"), run(-93578, 447));
        assert_eq!(String::from("Possible"), run(-1, -5));
        assert_eq!(String::from("Impossible"), run(-8, -3));
        assert_eq!(String::from("Possible"), run(0, 0));
    }
}
