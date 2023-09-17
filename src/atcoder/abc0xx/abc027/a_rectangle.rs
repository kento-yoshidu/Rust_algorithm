// https://atcoder.jp/contests/abc027/tasks/abc027_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    a ^ b ^ c
}

pub fn run2(a: usize, b: usize, c: usize) -> usize {
    if a == b && b == c {
        return a
    }

    if a == b && b != c {
        c
    } else if a != b && b == c {
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
        assert_eq!(2, run(1, 1, 2));
        assert_eq!(3, run(4, 3, 4));
        assert_eq!(5, run(5, 5, 5));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(1, 1, 2));
        assert_eq!(3, run2(4, 3, 4));
        assert_eq!(5, run2(5, 5, 5));
    }
}
