// https://atcoder.jp/contests/abc124/tasks/abc124_b

pub fn run(_n: i32, vec: Vec<i32>) -> i32 {
    let mut height = vec[0];

    let mut ans = 0;

    for h in vec.iter() {
        if height <= *h {
            ans += 1;
            height = *h;
        }
    }

    ans
}

pub fn run2(_n: usize, vec: Vec<usize>) -> usize {
    vec.iter()
        .fold((0, 0), |(count, state), height| {
            if state <= *height {
                (count + 1, *height)
            } else {
                (count, state)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, vec![6, 5, 6, 8]));
        assert_eq!(3, run(5, vec![4, 5, 3, 5, 4]));
        assert_eq!(1, run(5, vec![9, 5, 6, 8, 4]));
        assert_eq!(4, run(5, vec![1, 2, 3, 9, 4]));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(4, vec![6, 5, 6, 8]));
        assert_eq!(3, run2(5, vec![4, 5, 3, 5, 4]));
        assert_eq!(1, run2(5, vec![9, 5, 6, 8, 4]));
        assert_eq!(4, run2(5, vec![1, 2, 3, 9, 4]));
    }
}
