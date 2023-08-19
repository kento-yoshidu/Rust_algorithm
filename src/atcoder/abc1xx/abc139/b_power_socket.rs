// https://atcoder.jp/contests/abc139/tasks/abc139_b

fn run(a: usize, b: usize) -> usize {
    if a == b {
        1
    } else {
        b / a + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, 10));
        assert_eq!(2, run(8, 9));
        assert_eq!(1, run(8, 8));
    }
}
