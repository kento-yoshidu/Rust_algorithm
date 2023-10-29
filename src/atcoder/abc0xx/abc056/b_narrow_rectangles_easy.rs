// https://atcoder.jp/contests/abc056/tasks/abc056_b

pub fn run(w: i32, a: i32, b: i32) -> i32 {
    if (a - b).abs() <= w {
        0
    } else {
        (a - b).abs() - w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 2, 6));
        assert_eq!(0, run(3, 1, 3));
        assert_eq!(4, run(5, 10, 1));
    }
}
