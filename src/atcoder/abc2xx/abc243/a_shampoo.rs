// https://atcoder.jp/contests/abc243/tasks/abc243_a

pub fn run(v: isize, a: isize, b: isize, c: isize) -> String {
    let rest = v % (a + b + c);

    if rest - a < 0 {
        String::from("F")
    } else if rest - (a + b) < 0 {
        String::from("M")
    } else {
        String::from("T")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("T"), run(25, 10, 11, 12));
        assert_eq!(String::from("F"), run(30, 10, 10, 10));
        assert_eq!(String::from("M"), run(100000, 1, 1, 1))
    }
}

