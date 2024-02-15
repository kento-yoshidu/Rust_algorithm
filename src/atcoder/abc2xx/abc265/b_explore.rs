// https://atcoder.jp/contests/abc265/tasks/abc265_b

pub fn run(n: usize, _m: usize, t: usize, a: Vec<usize>, xy: Vec<(usize, usize)>) -> &'static str {
    let mut life_up = vec![0; n];

    for (x, y) in xy {
        life_up[x-1] = y;
    }

    let mut time = t;

    for (i, num) in a.iter().enumerate() {
        time += life_up[i];

        if time <= *num {
            return "No";
        } else {
            time -= num;
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<(usize, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 1, 10, vec![5, 7, 5], vec![(2, 10)], "Yes"),
            TestCase(4, 1, 10, vec![10, 7, 5], vec![(2, 10)], "No"),
            TestCase(2, 0, 2, vec![1], vec![], "Yes"),
        ];

        for TestCase(n, m, t, a, xy, expected) in tests {
            assert_eq!(run(n, m, t, a, xy), expected);
        }
    }
}
