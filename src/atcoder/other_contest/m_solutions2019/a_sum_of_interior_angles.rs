// https://atcoder.jp/contests/m-solutions2019/tasks/m_solutions2019_a

pub fn run(n: usize) -> usize {
    (n - 2) * 180
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(180, run(3));
        assert_eq!(17640, run(100));
    }
}
