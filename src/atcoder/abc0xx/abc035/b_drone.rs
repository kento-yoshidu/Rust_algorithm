// https://atcoder.jp/contests/abc035/tasks/abc035_b

fn run(s: &str, t: usize) -> i32 {
    let (v, p, k) = s.chars()
        .fold((0_i32, 0_i32, 0_i32), |acc, c| {
            match c {
                'U' => (acc.0+1, acc.1, acc.2),
                'D' => (acc.0-1, acc.1, acc.2),
                'L' => (acc.0, acc.1-1, acc.2),
                'R' => (acc.0, acc.1+1, acc.2),
                '?' => (acc.0, acc.1, acc.2+1),
                _ => unreachable!(),
            }
        });

    let dis = v.abs() + p.abs();

    match t {
        1 => dis + k as i32,
        2 => {
            if dis < k {
                if (k - dis) % 2 == 0 {
                    0
                } else {
                    1
                }
            } else {
                dis - k
            }
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, i32);

    #[test]
    fn abc035_b() {
        let tests = [
            TestCase("UL?", 1, 3),
            TestCase("UD?", 1, 1),
            TestCase("UUUU?DDR?LLLL", 1, 7),
            TestCase("UULL?", 2, 3),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}

// https://kenkoro.hatenablog.com/entry/2019/04/15/070655
