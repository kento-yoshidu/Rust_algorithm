// https://atcoder.jp/contests/abc324/tasks/abc324_d

fn run(n: usize, s: &str) -> isize {
    let mut target = [0isize; 10];

    for b in s.bytes() {
        target[(b - b'0') as usize] += 1;
    }

    let mut ans = 0;

    for i in 0u64.. {
        let sq = i * i;

        let mut counts = [0isize; 10];
        let mut tmp = sq;
        let mut digit_len = 0usize;

        if tmp == 0 {
            counts[0] = 1;
            digit_len = 1;
        } else {
            while tmp > 0 {
                counts[(tmp % 10) as usize] += 1;
                tmp /= 10;
                digit_len += 1;
            }
        }

        if digit_len > n {
            break;
        }

        counts[0] += (n - digit_len) as isize;

        if counts == target {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, isize);

    #[test]
    fn abc324_d() {
        let tests = [
            TestCase(4, "4320", 2),
            TestCase(3, "010", 2),
            TestCase(13, "8694027811503", 840),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
