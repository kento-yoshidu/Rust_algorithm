// https://atcoder.jp/contests/abc169/tasks/abc169_b

pub fn run(_n: usize, a: Vec<isize>) -> isize {
    if a.contains(&0) {
        return 0
    }

    let mut ans: isize = 1;

    for num in a {
        if ans * num > 10_isize.pow(18_u32) {
            ans = -1;
            break;
        } else {
            ans *= num;
        }
    }

    ans as isize
}

pub fn run2(_n: usize, a: Vec<isize>) -> isize {
    a.iter()
        .fold(1, |state, num| {
            if state * num > 10_usize.pow(18_u32) as isize {
                -1
            } else {
                state * num
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1000000000000000000, run(2, vec![1000000000, 1000000000]));
        assert_eq!(-1, run(3, vec![101, 9901, 999999000001]));
        assert_eq!(0, run(31, vec![4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0]));
    }
}
