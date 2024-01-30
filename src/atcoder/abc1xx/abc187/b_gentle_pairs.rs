// https://atcoder.jp/contests/abc187/tasks/abc187_b

pub fn run(n: usize, xy: Vec<(isize, isize)>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        let x1 = xy[i].0;
        let y1 = xy[i].1;

        for j in i+1..n {
            let x2 = xy[j].0;
            let y2 = xy[j].1;

            let d = (y2 - y1) as f64 / (x2 - x1) as f64;

            if -1.0 <= d && d <= 1.0 {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, vec![(0, 0), (1, 2), (2, 1)]));
        assert_eq!(0, run(1, vec![(-691, 273)]));
        assert_eq!(11, run(10, vec![(-31, -35), (8, -36), (22, 64), (5, 73), (-14, 8), (18, -58), (-41, -85), (1, -88), (-21, -85), (-11, 82)]));
    }
}
