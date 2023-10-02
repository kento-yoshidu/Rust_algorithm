// https://atcoder.jp/contests/abc144/tasks/abc144_c

pub fn run(n: usize) -> usize {
    let mut ans = n;

    for i in 1..n {
        if i*i > n {
            break
        }

        if n % i == 0 {
            let j = n / i;

            // (1, 1)からスタートするため、2を引く
            let num = i+j-2;

            ans = ans.min(num);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(10));
        assert_eq!(13, run(50));
        assert_eq!(10000000018, run(10000000019));
    }
}
