// https://atcoder.jp/contests/tenka1-2012-qualb/tasks/tenka1_2012_5

#[allow(unused)]
pub fn run(a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();

    for i in 0..=127 {
        if i % 3 == a && i % 5 == b && i % 7 == c {
            ans.push(i);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![23], run(2, 3, 2));
        assert_eq!(vec![1, 106], run(1, 1, 1));
        assert_eq!(vec![104], run(2, 4, 6));
    }
}
