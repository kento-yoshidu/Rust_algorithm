// https://atcoder.jp/contests/abc203/tasks/abc203_a

fn run(a: usize, b: usize, c: usize) -> usize {
    if a != b && b != c && a != c {
        return 0;
    }

    if a == b {
        c
    } else if b == c {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(2, 5, 2));
        assert_eq!(0, run(4, 5 ,6));
        assert_eq!(1, run(1, 1, 1));
    }
}
