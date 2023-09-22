// https://atcoder.jp/contests/abc319/tasks/abc319_b

pub fn run(n: usize) -> String {
    let mut ans = String::from("");

    'outer: for i in 0..=n {
        for j in 1..=9 {
            if n % j == 0 {
                if i % (n / j) == 0 {
                    ans.push(std::char::from_digit(j as u32, 10).unwrap());
                    // 最小のものを出力するので、見つかった時点でiを次に進める
                    continue 'outer
                }
            }
        }

        ans.push('-');
    }

    ans.chars().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("1-643-2-346-1"), run(12));
        assert_eq!(String::from("17777771"), run(7));
        assert_eq!(String::from("11"), run(1));
    }
}
