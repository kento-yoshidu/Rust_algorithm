// https://atcoder.jp/contests/abc210/tasks/abc210_a

pub fn run(n: usize, a: usize, x: usize, y: usize) -> usize {
    let over = if a >= n {
        0
    } else {
        n - a
    };

    a * x + over * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(90, run(5, 3, 20, 15));
        assert_eq!(1000, run(10, 10, 100, 1));
    }
}
