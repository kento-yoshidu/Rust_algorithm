// https://atcoder.jp/contests/abc324/tasks/abc324_d

fn run(n: usize, s: &str) -> i32 {
    let mut num: Vec<char> = s.chars().collect();
    num.sort();

    let mut ans = 0;

    for i in 0..3200000 {
        if (i as u128 * i as u128).to_string().len() > n {
            break
        }

        let mut str: Vec<char> = (i as u128 * i as u128).to_string().chars().collect();

        while str.len() < num.len() {
            str.push('0');
        }

        str.sort();

        if str == num {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, "4320"));
        assert_eq!(2, run(3, "010"));
        assert_eq!(840, run(13, "8694027811503"));
    }
}
