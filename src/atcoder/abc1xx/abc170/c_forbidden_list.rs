// https://atcoder.jp/contests/abc170/tasks/abc170_c

pub fn run(x: i32, _n: i32, vec: Vec<i32>) -> i32 {
    let mut ans = 101;
    let mut diff = 101;

    for i in 0..=20 {
        if vec.contains(&i) {
            continue
        }

        let dis = (i - x).abs();

        if diff > dis {
            ans = i;
            diff = dis;
        }
    }

    ans
}

pub fn run2(x: i32, _n: i32, vec: Vec<i32>) -> i32 {
    (0..101)
        .filter(|i| !vec.contains(i))
        .min_by_key(|&i| ((x-i).abs(), i))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(6, 5, vec![4, 7, 10, 6, 5]));
        assert_eq!(9, run(10, 5, vec![4, 7, 10, 6, 5]));
        assert_eq!(6, run(6, 4, vec![4, 7, 10, 5]));
    }

    #[test]
    fn test2() {
        assert_eq!(8, run2(6, 5, vec![4, 7, 10, 6, 5]));
        assert_eq!(9, run2(10, 5, vec![4, 7, 10, 6, 5]));
    }
}
