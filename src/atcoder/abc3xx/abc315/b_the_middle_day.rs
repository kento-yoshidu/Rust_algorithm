// https://atcoder.jp/contests/abc315/tasks/abc315_b

pub fn run(_m: i32, d: Vec<i32>) -> (usize, usize) {
    let mut middle: i32 = d.iter().sum::<i32>() / 2 + 1;

    for (index, d) in d.iter().enumerate() {
        if middle - d <= 0 {
            return (index+1, middle as usize)
        }

        middle -= d;
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((7, 2), run(12, vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]));
        assert_eq!((1, 1), run(1, vec![1]));
        assert_eq!((5, 3), run(6, vec![3, 1, 4, 1, 5, 9]));
    }
}
