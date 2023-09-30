// https://atcoder.jp/contests/abc129/tasks/abc129_b

pub fn run(n: i32, vec: Vec<i32>) -> i32 {
    let mut ans = 100;

    for i in 1..n {
        let mut l = 0;
        let mut r = 0;

        for (index, v) in vec.iter().enumerate() {
            if i >= index.try_into().unwrap() {
                l += v
            } else {
                r += v
            }
        }

        ans = ans.min(((l-r) as i32).abs())
    }

    ans
}

pub fn run2(n: i32, vec: Vec<i32>) -> i32 {
    let mut ans = 100;

    for i in 1..n {
        let mut tmp = 0;

        for (index, v) in vec.iter().enumerate() {
            if i > index.try_into().unwrap() {
                println!("l={}", v);
                tmp += v;
            } else {
                println!("r={}", v);
                tmp -= v;
            }
        }

        ans = ans.min(tmp.abs());
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, run(3, vec![1, 2, 3]));
        assert_eq!(2, run(4, vec![1, 3, 1, 1]));
        assert_eq!(2, run(8, vec![27, 23, 76, 2, 3, 5, 62, 52]));
    }

    #[test]
    fn test2() {
        assert_eq!(0, run2(3, vec![1, 2, 3]));
        assert_eq!(2, run2(4, vec![1, 3, 1, 1]));
        assert_eq!(2, run2(8, vec![27, 23, 76, 2, 3, 5, 62, 52]));
    }
}
