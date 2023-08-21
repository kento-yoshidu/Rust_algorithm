// https://atcoder.jp/contests/abc255/tasks/abc255_a

pub fn run(r: usize, c: usize, arr: [[usize; 2]; 2]) -> usize {
    arr[r-1][c-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, run(1, 2, [[1, 0], [0, 1]]));
        assert_eq!(4, run(2, 2, [[1, 2], [3, 4]]));
        assert_eq!(70, run(2, 1, [[90, 80], [70, 60]]));
    }
}
