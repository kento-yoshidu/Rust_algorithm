// https://atcoder.jp/contests/abc068/tasks/abc068_b

fn calc(n: usize, count: usize) -> usize {
    if n % 2 != 0 {
        return count
    } else {
        calc(n / 2, count+1)
    }
}

pub fn run(n: usize) -> usize {
    (1..=n)
        .skip(1)
        .step_by(2)
        .map(|i| {
            (i, calc(i, 0))
        })
        .filter(|t| {
            t.1 != 0
        })
        .max_by_key(|t| {
            t.1
        })
        .unwrap_or((1, 0)).0
}

fn calc2(n: usize) -> bool {
    if n / 2 == 0 {
        return true
    }

    if n % 2 != 0 {
        return false
    }

    calc2(n / 2)
}

pub fn run2(n: usize) -> usize {
    (1..=n)
        .rev()
        .find(|num| {
            calc2(*num)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(7));
        assert_eq!(32, run(32));
        assert_eq!(1, run(1));
        assert_eq!(64, run(100));
    }

    #[test]
    fn test2() {
        assert_eq!(4, run2(7));
        assert_eq!(32, run2(32));
        assert_eq!(1, run2(1));
        assert_eq!(64, run2(100));
    }
}
