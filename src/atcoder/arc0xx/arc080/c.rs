// https://atcoder.jp/contests/abc069/tasks/arc080_a

fn run(n: usize, a: Vec<usize>) -> &'static str {
    let mut x = 0;
    let mut _y = 0;
    let mut z = 0;

    for num in a {
        if num % 4 == 0 {
            x += 1;
        } else if num % 2 == 0 {
            _y += 1;
        } else {
            z += 1;
        }
    }

    if x + 1 == z && n == x + z {
        "Yes"
    } else if x < z {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn arc080_c() {
        let tests = [
            TestCase(3, vec![1, 10, 100], "Yes"),
            TestCase(4, vec![1, 2, 3, 4], "No"),
            TestCase(3, vec![1, 4, 1], "Yes"),
            TestCase(2, vec![1, 1], "No"),
            TestCase(6, vec![2, 7, 1, 8, 2, 8], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}

