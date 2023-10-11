// https://atcoder.jp/contests/abc297/tasks/abc297_a

pub fn run(n: i32, d: i32, t: Vec<i32>) -> i32 {
    for i in 0..n-1 {
        if t[i as usize + 1] - t[i as usize] <= d {
            return t[i as usize + 1];
        }
    }

    -1
}

/*
pub fn run2(_n: isize, d: isize, t: Vec<isize>) -> String {
    t.windows(2).find(|v| {
        v[1] - v[0] <= d
    })
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1300, run(4, 500, vec![300, 900, 1300, 1700]));
        assert_eq!(-1, run(5, 99, vec![100, 200, 300, 400, 500]));
        assert_eq!(600, run(4, 500, vec![100, 600, 1100, 1600]));
    }

    /*
    #[test]
    fn test2() {
        assert_eq!(String::from("1300"), run2(4, 500, vec![300, 900, 1300, 1700]));
        assert_eq!(String::from("-1"), run2(5, 99, vec![100, 200, 300, 400, 500]));
        assert_eq!(String::from("600"), run2(4, 500, vec![100, 600, 1100, 1600]));
    }
    */
}
