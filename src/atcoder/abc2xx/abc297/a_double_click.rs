// https://atcoder.jp/contests/abc297/tasks/abc297_a

pub fn run(n: i32, d: i32, t: Vec<i32>) -> i32 {
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
        assert_eq!(1300, run(4, 500, vec![300, 900, 1300, 1700]));
        assert_eq!(-1, run(5, 99, vec![100, 200, 300, 400, 500]));
        assert_eq!(600, run(4, 500, vec![100, 600, 1100, 1600]));
    }
}
