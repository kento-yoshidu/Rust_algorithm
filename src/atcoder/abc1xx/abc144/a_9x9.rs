// https://atcoder.jp/contests/abc144/tasks/abc144_a

pub fn run(a: isize, b: isize) ->isize {
    if a > 9 || b > 9 {
        -1
    } else {
        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(2, 5));
        assert_eq!(-1, run(5, 10));
        assert_eq!(81, run(9, 9));
    }
}
