// https://atcoder.jp/contests/arc177/tasks/arc177_a

fn run(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, _n: usize, x: Vec<usize>) -> &'static str {
    let mut vec = vec![a, b, c, d, e, f];

    for price in x {
        let mut rest = price;

        while rest >= 500 && vec[5] >= 1 {
            rest -= 500;
            vec[5] -= 1;
        }

        while rest >= 100 && vec[4] >= 1 {
            rest -= 100;
            vec[4] -= 1;
        }

        while rest >= 50 && vec[3] >= 1 {
            rest -= 50;
            vec[3] -= 1;
        }

        while rest >= 10 && vec[2] >= 1 {
            rest -= 10;
            vec[2] -= 1;
        }

        while rest >= 5 && vec[1] >= 1 {
            rest -= 5;
            vec[1] -= 1;
        }

        while rest >= 1 && vec[0] >= 1 {
            rest -= 1;
            vec[0] -= 1;
        }

        if rest != 0 {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 0, 6, 3, 4, 1, 3, vec![700, 250, 160], "Yes"),
            TestCase(0, 0, 0, 2, 4, 0, 3, vec![100, 200, 300], "No"),
            TestCase(0, 0, 0, 0, 8, 8, 1, vec![250], "No"),
            TestCase(20, 5, 9, 7, 10, 6, 5, vec![177, 177, 177, 177, 177], "Yes"),
            TestCase(17, 5, 9, 7, 10, 6, 5, vec![177, 177, 177, 177, 177], "No"),
        ];

        for TestCase(a, b, c, d, e, f, n, x, expected) in tests {
            assert_eq!(run(a, b, c, d, e, f, n, x), expected);
        }
    }
}
