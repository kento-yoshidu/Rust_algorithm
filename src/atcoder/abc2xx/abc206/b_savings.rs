// https://atcoder.jp/contests/abc206/tasks/abc206_b

pub fn run(num: usize) -> usize {
    let mut total = 0;
    let mut day = 0;

    for i in 1..=num {
        if total < num {
            total += i;
            day += 1;
        } else {
            break;
        }
    }

    day
}

fn check(count: isize, rest: isize) -> isize {
    if rest - count <= 0 {
        count
    } else {
        check(count+1, rest - count)
    }
}

fn run2(n: isize) -> isize {
    check(1, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(12));
        assert_eq!(447, run(100128));
    }

    #[test]
    fn test2() {
        assert_eq!(5, run2(12));
        assert_eq!(447, run2(100128));
    }
}
