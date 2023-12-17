// https://atcoder.jp/contests/abc282/tasks/abc282_b

pub fn run(n: usize, m: usize, vec: Vec<&str>) -> i32 {
    let mut ans = 0;

    for i in 0..n-1 {
        'outer: for j in i+1..n {

            for index in 0..m {
                if vec[i].chars().nth(index).unwrap() == 'x' && vec[j].chars().nth(index).unwrap() == 'x' {
                    break 'outer;
                }
            }

            ans += 1;
        }
    }

    ans
}

pub fn run2(_u: usize, _m: usize, vec: Vec<&str>) -> usize {
    use itertools::Itertools;

    vec.iter()
        .combinations(2)
        .filter(|t| {
            t[0].chars()
                .zip(t[1].chars())
                .all(|(c1, c2)| {
                    c1 == 'o' || c2 == 'o'
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(5, 5, vec!["ooooo", "oooxx", "xxooo", "oxoxo", "xxxxx"]));
        assert_eq!(1, run(3, 2, vec!["ox", "xo", "xx"]));
        assert_eq!(0, run(2, 4, vec!["xxxx", "oxox"]));
    }

    #[test]
    fn test2() {
        assert_eq!(5, run2(5, 5, vec!["ooooo", "oooxx", "xxooo", "oxoxo", "xxxxx"]));
        assert_eq!(1, run(3, 2, vec!["ox", "xo", "xx"]));
        assert_eq!(0, run(2, 4, vec!["xxxx", "oxox"]));
    }
}
