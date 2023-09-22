// https://atcoder.jp/contests/abc273/tasks/abc273_a

fn calc(n: usize) -> usize {
    if n == 0 {
        return 1
    }

    n * calc(n - 1)
}

pub fn run(n: usize) -> usize {
    calc(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3628800, run(10));
    }
}
