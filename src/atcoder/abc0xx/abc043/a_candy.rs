// https://atcoder.jp/contests/abc043/tasks/abc043_a

fn func(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        func(n - 1) + n
    }
}

#[allow(dead_code)]
pub fn run(n: i32) -> i32 {
    func(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(3));
        assert_eq!(55, run(10));
        assert_eq!(1, run(1));
    }
}
