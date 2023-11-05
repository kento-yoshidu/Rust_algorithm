// https://atcoder.jp/contests/abc090/tasks/abc090_b

pub fn run(a: i32, b: i32) -> i32 {
    let mut ans = 0;

    for i in a..=b {
        let i1 = i / 10000;
        let i2 = i / 1000 % 10;
        let i3 = i / 10 % 10;
        let i4 = i % 10;

        if i1 == i4 && i2 == i3 {
            ans += 1;
        }
    }

    ans
}

fn calc(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

pub fn run2(a: usize, b: usize) -> usize {
    (a..=b)
        .filter(|num| {
            calc(num.to_string())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(11009, 11332));
        assert_eq!(612, run(31415, 92653))
    }

    #[test]
    fn test2() {
        assert_eq!(4, run2(11009, 11332));
        assert_eq!(612, run2(31415, 92653))
    }
}
