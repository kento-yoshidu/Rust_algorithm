// https://atcoder.jp/contests/abc100/tasks/abc100_c

pub fn run(n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();
    let mut ans = 0;

    for i in 0..n {
        loop {
            if vec[i] % 2 == 0 {
                vec[i] /= 2;
                ans += 1;
            } else {
                break
            }
        }
    }

    ans
}

fn calc(num: usize, count: usize) -> usize {
    if num % 2 != 0 {
        count
    } else {
        calc(num/2, count+1)
    }
}

pub fn run2(_n: usize, a: Vec<usize>) -> usize {
    a.iter()
        .map(|num| {
            // 各要素が2で何回割り切れるかを合計
            calc(*num, 0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(3, vec![5, 2, 4]));
        assert_eq!(0, run(4, vec![631, 577, 243, 199]));
        assert_eq!(39, run(10, vec![2184, 2126, 1721, 1800, 1024, 2528, 3360, 1945, 1280, 1776]));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(3, vec![5, 2, 4]));
        assert_eq!(0, run2(4, vec![631, 577, 243, 199]));
        assert_eq!(39, run2(10, vec![2184, 2126, 1721, 1800, 1024, 2528, 3360, 1945, 1280, 1776]));
    }
}
