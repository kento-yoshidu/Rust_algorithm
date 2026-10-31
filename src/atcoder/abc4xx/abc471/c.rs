// https://atcoder.jp/contests/abc471/tasks/abc471_c

fn run(_n: usize, a: Vec<isize>) -> isize {
    let mut p = Vec::new();
    let mut m = Vec::new();

    for a in a {
        if a > 0 {
            p.push(a);
        } else {
            m.push(a);
        }
    }

    p.sort();
    m.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;

    let mut p_pos = 0;
    let mut m_pos = 0;

    let mut cur = 0;

    loop {
        let has_p = p_pos < p.len();
        let has_m = m_pos < m.len();

        if !has_p && !has_m {
            return ans;
        }

        if !has_m {
            ans += (cur - p[p_pos]).abs();
            cur = p[p_pos];
            p_pos += 1;
            continue;
        }

        if !has_p {
            ans += (cur - m[m_pos]).abs();
            cur = m[m_pos];
            m_pos += 1;
            continue;
        }

        if (cur - p[p_pos]).abs() < (cur - m[m_pos]).abs() {
            ans += (cur - p[p_pos]).abs();
            cur = p[p_pos];
            p_pos += 1;
        } else {
            ans += (cur - m[m_pos]).abs();
            cur = m[m_pos];
            m_pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc471_c() {
        let tests = [
            TestCase(4, vec![-1, -4, 2, -11], 23),
            TestCase(10, vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -6], 17),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
