// https://atcoder.jp/contests/abc224/tasks/abc224_b

pub fn run(h: usize, w: usize, a: Vec<Vec<usize>>) -> &'static str {
    for i1 in 0..h {
        for i2 in i1..h {
            for j1 in 0..w {
                for j2 in j1..w {
                    if a[i1][j1] + a[i2][j2] > a[i2][j1] + a[i1][j2] {
                        return "No";
                    }
                }
            }
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec![vec![2, 1, 4], vec![3, 1, 3], vec![6, 4, 1]], "Yes"),
            TestCase(2, 4, vec![vec![4, 3, 2, 1], vec![5, 6, 7, 8]], "No"),
            TestCase(2, 2, vec![vec![1, 1], vec![1, 1]], "Yes"),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}
