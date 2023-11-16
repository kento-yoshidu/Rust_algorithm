// https://atcoder.jp/contests/abc154/tasks/abc154_a

pub fn run(s: &str, _t: &str, a: usize, b: usize, u: &str) -> Vec<usize> {
    if s == u {
        vec![a-1, b]
    } else {
        vec![a, b-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 4], run("red", "blue", 3, 4, "red"));
        assert_eq!(vec![5, 4], run("red", "blue", 5, 5, "blue"));
    }
}
