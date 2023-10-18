// https://atcoder.jp/contests/abc052/tasks/abc052_b

pub fn run(_n: usize, s: String) -> isize {
    let mut ans = 0;
    let mut tmp = 0;

    s.chars().for_each(|c| {
        match c {
            'I' => {
                tmp += 1;
                ans = ans.max(tmp);
            },
            'D' => {
                tmp -= 1;
            },
            _ => unreachable!(),
        }
    });

    ans
}

pub fn run2(_n: usize, s: String) -> isize {
    s.chars().fold((0, 0), |(tmp, max), c| {
        match c {
            'I' => (tmp+1, max.max(tmp+1)),
            'D' => (tmp-1, max),
            _ => unreachable!(),
        }
    }).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(5, String::from("IIDID")));
        assert_eq!(0, run(7, String::from("DDIDDII")));
        assert_eq!(0, run(7, String::from("DDDDDDD")));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(5, String::from("IIDID")));
        assert_eq!(0, run2(7, String::from("DDIDDII")));
        assert_eq!(0, run2(7, String::from("DDDDDDD")));
    }
}
