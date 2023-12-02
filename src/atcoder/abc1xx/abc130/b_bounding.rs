// https://atcoder.jp/contests/abc130/tasks/abc130_b

pub fn run(x: i32, vec: Vec<i32>) -> i32 {
    let mut current = 0;
    let mut ans = 0;

    for i in vec.iter() {
        if current <= x {
            current += i;
            ans += 1;
        } else {
            break;
        }
    }

    ans
}

pub fn run2(_n: usize, x: usize, l: Vec<usize>) -> usize {
    l.iter()
        .scan(0, |state, d| {
            *state += d;
            Some(*state)
        })
        .filter(|d| {
            *d <= x
        })
        .count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, vec![3, 4, 5]));
        assert_eq!(4, run(9, vec![3, 3, 3, 3]));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(3, 6, vec![3, 4, 5]));
        assert_eq!(4, run2(4, 9, vec![3, 3, 3, 3]));
    }
}
