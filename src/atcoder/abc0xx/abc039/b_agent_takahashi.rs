// https://atcoder.jp/contests/abc039/tasks/abc039_b

pub fn run(n: usize) -> usize {
    for i in 1..=1000 {
        if i*i*i*i == n {
            return i;
        }
    }

    unreachable!();
}

pub fn run2(n: usize) -> usize {
    ((n as f64).sqrt()).sqrt() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(1));
        assert_eq!(25, run(390625));
        assert_eq!(177, run(981506241));
    }
}
