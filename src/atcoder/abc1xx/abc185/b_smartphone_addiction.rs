// https://atcoder.jp/contests/abc185/tasks/abc185_b

pub fn run(n: isize, _m: isize, t: isize, ab: Vec<(isize, isize)>) -> String {
    let mut rest = n;
    let mut start = 0;

    for (a, b) in ab {
        if rest - (a - start) <= 0 {
            return String::from("No")
        };

        rest -= a - start;

        if rest + (b - a) > n {
            rest = n;
        } else {
            rest += b - a;
        }

        start = b;
    }

    if rest <= t - start {
        String::from("No")
    } else {
        String::from("Yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(10, 2, 20, vec![(9, 11), (13, 17)]));
        assert_eq!(String::from("No"), run(10, 2, 20, vec![(9, 11), (13, 16)]));
        assert_eq!(String::from("Yes"), run(15, 3, 30, vec![(5, 8), (15, 17), (24, 27)]));
        assert_eq!(String::from("No"), run(20, 1, 30, vec![(20, 29)]));
        assert_eq!(String::from("No"), run(20, 1, 30, vec![(1, 10)]));
    }
}
