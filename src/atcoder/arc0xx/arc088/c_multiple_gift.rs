// https://atcoder.jp/contests/abc083/tasks/arc088_a

fn calc(n: usize, y: usize, count: usize) -> usize {
    if n > y {
        count
    } else {
        calc(n*2, y, count+1)
    }
}

pub fn run(x: usize, y: usize) -> usize {
    calc(x, y, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(3, 20));
        assert_eq!(3, run(25, 100));
        assert_eq!(31, run(314159265, 358979323846264338));
    }
}
