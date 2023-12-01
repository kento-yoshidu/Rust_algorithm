// https://atcoder.jp/contests/abc235/tasks/abc235_b

pub fn run(n: usize, h: Vec<usize>) -> usize {
    let mut ans = h[0];

    for i in 1..n {
        if ans < h[i] {
            ans = h[i]
        } else {
            return ans;
        }
    }

    h[n-1]
}

pub fn run2(n: usize, h: Vec<usize>) -> usize {
    h.windows(2)
        .find(|a| {
            a[0] >= a[1]
        })
        .map(|a| a[0])
        .unwrap_or(h[n-1])
}

fn calc(i: usize, vec: Vec<usize>) -> usize {
    if  i == vec.len()-1 || vec[i] >= vec[i+1] {
        vec[i]
    } else {
        calc(i+1, vec)
    }
}

pub fn run3(_n: usize, h: Vec<usize>) -> usize {
    calc(0, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(5, vec![1, 5, 10, 4, 2]));
        assert_eq!(100000, run(3, vec![100, 1000, 100000]));
        assert_eq!(1828, run(4, vec![27, 1828, 1828, 9242]));
    }

    #[test]
    fn test2() {
        assert_eq!(10, run2(5, vec![1, 5, 10, 4, 2]));
        assert_eq!(100000, run2(3, vec![100, 1000, 100000]));
        assert_eq!(1828, run2(4, vec![27, 1828, 1828, 9242]));
    }
    #[test]
    fn test3() {
        assert_eq!(10, run3(5, vec![1, 5, 10, 4, 2]));
        assert_eq!(100000, run3(3, vec![100, 1000, 100000]));
        assert_eq!(1828, run3(4, vec![27, 1828, 1828, 9242]));
    }
}
