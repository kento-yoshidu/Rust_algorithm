// https://atcoder.jp/contests/abc079/tasks/abc079_c

fn run(s: &str) -> String {
    let nums: Vec<i32> = s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    for bit in 0..(1 << 3) {
        let mut ans = Vec::new();
        let mut sum = nums[0];

        for i in 0..3 {
            if bit & (1 << i) != 0 {
                sum += nums[i+1];
                ans.push('+');
            } else {
                sum -= nums[i+1];
                ans.push('-');
            }
        }

        if sum == 7 {
            return format!("{}{}{}{}{}{}{}=7", nums[0], ans[0], nums[1], ans[1], nums[2], ans[2], nums[3]);
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("1+2+2+2=7"), run("1222"));
        assert_eq!(String::from("0-2+9-0=7"), run("0290"));
        assert_eq!(String::from("3+2+4-2=7"), run("3242"));
    }
}
