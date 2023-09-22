// https://atcoder.jp/contests/abc088/tasks/abc088_b

pub fn run(_n: i32, vec: &mut Vec<i32>) -> i32 {
    vec.sort();

    let mut ans = 0;

    for (i, num) in vec.iter().rev().enumerate() {
        if i % 2 == 0 {
            ans += num
        } else {
            ans -= num
        }
    }

    ans
}

pub fn run2(_n: isize, vec: &mut Vec<isize>) -> isize {
    vec.sort();

    vec
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| {
            if i % 2 == 0 {
                *num
            } else {
                -num
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(2, &mut vec![3, 1]));
        assert_eq!(5, run(2, &mut vec![2, 7, 4]));
        assert_eq!(18, run(2, &mut vec![20, 18, 2, 18]));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(2, &mut vec![3, 1]));
        assert_eq!(5, run2(2, &mut vec![2, 7, 4]));
        assert_eq!(18, run2(2, &mut vec![20, 18, 2, 18]));
    }
}
