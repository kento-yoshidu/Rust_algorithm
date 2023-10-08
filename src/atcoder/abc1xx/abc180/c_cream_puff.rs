// https://atcoder.jp/contests/abc180/tasks/abc180_c

pub fn run(n: usize) -> Vec<usize> {
    let mut ans = Vec::new();

    for i in 1..n {
        if i*i > n {
            break
        }

        if n % i == 0 {
            let j = n / i;

            ans.push(i);
            ans.push(j);
        }
    }

    ans.sort();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 2, 3, 6], run(6));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 30, 36, 40, 45, 48, 60, 72, 80, 90, 120, 144, 180, 240, 360, 720], run(720));
        assert_eq!(vec![1, 1000000007], run(1000000007));
    }
}
