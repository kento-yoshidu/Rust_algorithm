// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/2/ITP1_2_A

pub fn run(a: usize, b: usize) -> &'static str {
    if a > b {
        "a > b"
    } else if a == b {
        "a = b"
    } else {
        "a < b"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("a < b", run(1, 2));
        assert_eq!("a > b", run(4, 3));
        assert_eq!("a = b", run(5, 5));
    }
}
