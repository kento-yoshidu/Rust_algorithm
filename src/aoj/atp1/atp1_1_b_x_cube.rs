// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_B

pub fn run(n: i32) -> i32 {
    n.pow(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(2));
        assert_eq!(27, run(3));
        assert_eq!(262144, run(64));
        assert_eq!(1000000, run(100));
    }
}
