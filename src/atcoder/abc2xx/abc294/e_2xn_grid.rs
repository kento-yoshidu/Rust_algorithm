// https://atcoder.jp/contests/abc294/tasks/abc294_e

pub fn run(l: usize, n1: usize, n2: usize, vl1: Vec<(usize, usize)>, vl2: Vec<(usize, usize)>) -> usize {
    let mut ans = 0;

    let mut c1_index = 0;
    let mut c2_index = 0;

    let mut a_rem = vl1[0].1;
    let mut b_rem = vl2[0].1;

    loop {
        if a_rem < b_rem {
            if vl1[c1_index].0 == vl2[c2_index].0 {
                ans += a_rem;
            }

            if c1_index+1 == n1 {
                break;
            }

            b_rem -= a_rem;
            c1_index += 1;
            a_rem = vl1[c1_index].1;
        } else if a_rem > b_rem {
            if vl1[c1_index].0 == vl2[c2_index].0 {
                ans += b_rem;
            }

            if c2_index+1 == n2 {
                break;
            }

            a_rem -= b_rem;
            c2_index += 1;
            b_rem = vl2[c2_index].1;
        } else {
            if vl1[c1_index].0 == vl2[c2_index].0 {
                ans += a_rem;
            }

            if c1_index+1 == n1 || c2_index+1 == n2 {
                break;
            }

            c1_index += 1;
            a_rem = vl1[c1_index].1;

            c2_index += 1;
            b_rem = vl2[c2_index].1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 4, 3, vec![(1, 2), (3, 2), (2, 3), (3, 1)], vec![(1, 4), (2, 1), (3, 3)], 4),
            TestCase(10000000000, 1, 1, vec![(1, 10000000000)], vec![(1, 10000000000)], 10000000000),
            TestCase(1000, 4, 7, vec![(19, 79), (33, 463), (19, 178), (33, 280)], vec![(19, 255), (33, 92), (34, 25), (19, 96), (12, 11), (19, 490), (33, 31)], 380),
            TestCase(1, 1, 1, vec![(593477330, 1)], vec![(294073833, 1)], 0),
            TestCase(1, 1, 1, vec![(983778696, 1)], vec![(983778696, 1)], 1),
        ];

        for TestCase(l, n1, n2, vl1, vl2, expected) in tests {
            assert_eq!(run(l, n1, n2, vl1, vl2), expected);
        }
    }
}
