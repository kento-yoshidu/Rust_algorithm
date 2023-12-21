// https://atcoder.jp/contests/abc156/tasks/abc156_b

fn calc(n: usize, base: usize) -> u32 {
    if n < base {
        return 1;
    } else {
        let boxed = Box::new(n);

        calc(*boxed / base, base) + 1
    }
}

pub fn run(n: usize, k: usize) -> u32 {
    calc(n, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(11, 2));
        assert_eq!(7, run(1010101, 10));
        assert_eq!(18, run(314159265, 3));
    }
}
