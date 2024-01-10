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

fn calc(rest: isize, i: usize, d: Vec<isize>) -> (usize, isize) {
    if rest - d[i] <= 0 {
        return (i + 1, rest);
    }

    calc(rest - d[i], i + 1, d)
}

pub fn run2(_m: isize, d: Vec<isize>) -> (usize, isize) {
    let rest = d.iter().sum::<isize>() / 2 + 1;

    calc(rest, 0, d)
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

    #[test]
    fn test2() {
        assert_eq!((7, 2), run2(12, vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]));
        assert_eq!((1, 1), run2(1, vec![1]));
        assert_eq!((5, 3), run2(6, vec![3, 1, 4, 1, 5, 9]));
    }
}
