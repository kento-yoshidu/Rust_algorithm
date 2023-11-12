// https://atcoder.jp/contests/abc114/tasks/abc114_b

pub fn run(s: String) -> i32 {
    let mut ans = 1000;

    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..(chars.len() - 2) {
        let num = (chars[i] as i32 - 48) * 100 + (chars[i+1] as i32 - 48) * 10 + (chars[i+2] as i32 -48);

        ans = ans.min((753 - num).abs());
    }

    ans
}

fn check(s: String) -> usize {
    (s.parse::<isize>().unwrap() - 753).abs() as usize
}

pub fn run2(s: String) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.windows(3)
        .map(|a| {
            check(format!("{}{}{}", a[0], a[1], a[2]))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(34, run(String::from("1234567876")));
        assert_eq!(0, run(String::from("35753")));
        assert_eq!(642, run(String::from("1111111111")));
    }

    #[test]
    fn test2() {
        assert_eq!(34, run2(String::from("1234567876")));
        assert_eq!(0, run2(String::from("35753")));
        assert_eq!(642, run2(String::from("1111111111")));
    }
}
