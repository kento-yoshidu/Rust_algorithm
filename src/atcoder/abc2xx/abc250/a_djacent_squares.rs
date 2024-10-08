// https://atcoder.jp/contests/abc250/tasks/abc250_a

pub fn run(h: usize, w: usize, r: usize, c: usize) -> usize {
    let mut ans = 0;

    if h != 1 {
        ans += 1;
    }
    if r != 1 && r != h {
        ans += 1;
    }

    if w != 1 {
        ans += 1;
    }
    if c != 1 && c != w {
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(3, 4, 2, 2));
        assert_eq!(3, run(3, 4, 1, 3));
        assert_eq!(2, run(3, 4, 3, 4));
        assert_eq!(0, run(1, 1, 1, 1));
    }
}

