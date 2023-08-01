// https://atcoder.jp/contests/abc078/tasks/abc078_b

#[allow(dead_code)]
pub fn run(x: i32, y: i32, z: i32) -> i32 {
    (x - z) / (y + z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        assert_eq!(3, run(13, 3, 1));
        assert_eq!(2, run(12, 3, 1));
        assert_eq!(49999, run(100000, 1, 1));
        assert_eq!(110, run(64146, 123, 456));
        assert_eq!(109, run(64145, 123, 456));
    }
}

