// https://atcoder.jp/contests/abc196/tasks/abc196_c

fn func(n: usize) -> usize {
    let mut num = 1;
    let mut nn= n;

    loop {
        num *= 10;
        nn /= 10;

        if nn == 0 {
            break
        }
    }

    n * num + n
}

#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    let mut ans = 0;

    let mut i = 1;

    loop {
        let num = func(i);

        if num <= n {
            ans += 1;
            i += 1;
        } else {
            break
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(33));
        assert_eq!(13, run(1333));
        assert_eq!(999, run(10000000));
    }
}
