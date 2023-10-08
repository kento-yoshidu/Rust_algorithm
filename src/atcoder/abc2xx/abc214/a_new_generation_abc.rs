// https://atcoder.jp/contests/abc214/tasks/abc214_a

pub fn run(n: usize) -> usize {
    if n >= 212 {
        8
    } else if n >= 126 {
        6
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(214));
        assert_eq!(6, run(126));
        assert_eq!(4, run(1));
    }
}
