// https://atcoder.jp/contests/abc167/tasks/abc167_b

pub fn run(a: isize, b: isize, _c: isize, k: isize) -> isize {
    if a >= k {
        k
    } else if a + b >= k {
        a
    } else {
        a - (k - (a + b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(2, 1, 1, 3));
        assert_eq!(0, run(1 ,2 ,3 ,4));
        assert_eq!(2000000000, run(2000000000, 0, 0, 2000000000));
        assert_eq!(-2000000000, run(0, 0, 2000000000, 2000000000));
    }
}
