// https://atcoder.jp/contests/abc317/tasks/abc317_a

pub fn run (_n: usize, h: usize, x: usize, p: Vec<usize>) -> usize {
    p.iter().position(|&num| {
        num >= (x - h)
    }).unwrap()+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 100, 200, vec![50, 200, 999]));
        assert_eq!(2, run(2, 10, 21, vec![10, 999]));
        assert_eq!(4, run(10, 500, 999, vec![38, 420, 490, 585, 613, 614, 760, 926, 945, 999]));
    }
}
