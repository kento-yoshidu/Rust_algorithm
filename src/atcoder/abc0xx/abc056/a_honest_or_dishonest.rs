// https://atcoder.jp/contests/abc056/tasks/abc056_a

pub fn run<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a == "H" {
        return b;
    }

    if b == "H" {
        "D"
    } else {
        "H"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("H", run("H", "H"));
        assert_eq!("D", run("D", "H"));
        assert_eq!("H", run("D", "D"));
    }
}
