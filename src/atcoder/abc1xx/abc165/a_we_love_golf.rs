// https://atcoder.jp/contests/abc165/tasks/abc165_a

pub fn run(k: u32, a: u32, b: u32) -> &'static str {
    for n in a..=b {
        if n % k == 0 {
            return "OK";
        }
    }

    "NG"
}

pub fn run2(k: usize, a: usize, b: usize) -> String {
    if (a..=b).any(|num| {
        num % k == 0
    }) {
        String::from("OK")
    } else {
        String::from("NG")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("OK", run(7, 500, 600));
        assert_eq!("NG", run(4, 5, 7));
        assert_eq!("OK", run(1, 11, 11));
    }

    #[test]
    fn test2() {
        assert_eq!("OK", run2(7, 500, 600));
        assert_eq!("NG", run2(4, 5, 7));
        assert_eq!("OK", run2(1, 11, 11));
    }
}
