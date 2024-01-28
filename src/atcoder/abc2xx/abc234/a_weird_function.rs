// https://atcoder.jp/contests/abc234/tasks/abc234_a

fn f(x: usize) -> usize {
    x * x + 2 * x + 3
}

pub fn run(t: usize) -> usize {
    f(f(f(t) + t) + f(f(t)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1371, run(0));
        assert_eq!(722502, run(3));
        assert_eq!(1111355571, run(10));
    }
}
