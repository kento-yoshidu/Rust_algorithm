#[allow(dead_code)]
pub fn run(n: i32, d: i32, t: &[i32]) -> i32 {
    for i in 0..n-1 {
        if t[i as usize + 1] - t[i as usize] <= d {
            return t[i as usize + 1];
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1300, run(4, 500, &[300, 900, 1300, 1700]));
        assert_eq!(-1, run(5, 99, &[100, 200, 300, 400, 500]));
        assert_eq!(600, run(4, 500, &[100, 600, 1100, 1600]));
    }
}
