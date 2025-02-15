// https://atcoder.jp/contests/abc086/tasks/abc086_b

use num_integer::Roots;

fn run(a: usize, b: usize) -> &'static str {
    let num = a + b;

    if num.sqrt().pow(2) == num {
        "Yes"
    }
    else {
        "No"
    }
}

fn run2(a: usize, b: usize) -> &'static str {
    let num = a + b;

    for i in 1..=(num.sqrt()) {
        if i * i == num {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 21, "Yes"),
            TestCase(1, 44, "Yes"),
            TestCase(1, 69, "Yes"),
            TestCase(100, 100, "No"),
            TestCase(12, 10, "No"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
