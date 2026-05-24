// https://atcoder.jp/contests/abc453/tasks/abc453_c

fn run(n: usize, l: Vec<isize>) -> usize {
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut pos = 0.5;
        let mut count = 0;

        for i in 0..n {
            let new_pos;

            if bit & (1 << i) != 0 {
                // +の方向に移動
                new_pos = pos + l[i] as f64;

                if pos < 0.0 && 0.0 < new_pos {
                    count += 1;
                }
            } else {
                // -の方向に移動
                new_pos = pos - l[i] as f64;

                if pos > 0.0 && new_pos < 0.0 {
                    count += 1;
                }
            }

            pos = new_pos;
        }

        ans = ans.max(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, usize);

    #[test]
    fn abc453_c() {
        let tests = [
            TestCase(5, vec![2, 5, 2, 2, 1], 4),
            TestCase(5, vec![100, 1, 2, 3, 4], 1),
            TestCase(20, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
