// https://atcoder.jp/contests/abc275/tasks/abc275_a

pub fn run(_n: usize, h: Vec<usize>) -> usize {
    let max = h.iter().max().unwrap();

    h.iter().position(|num| num == max).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, vec![50, 80, 70]));
        assert_eq!(1, run(1, vec![1000000000]));
        assert_eq!(9, run(10, vec![22, 75, 26, 45, 72, 81, 47, 29, 97, 2]));
    }
}
