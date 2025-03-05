// https://atcoder.jp/contests/abc380/tasks/abc380_a

use itertools::Itertools;

fn run(n: &str) -> &'static str {
    let (a, b, c) = n.chars()
        .fold((0, 0, 0), |acc, c| {
            match c {
                '1' => {
                    (acc.0+1, acc.1, acc.2)
                },
                '2' => {
                    (acc.0, acc.1+1, acc.2)
                },
                '3' => {
                    (acc.0, acc.1, acc.2+1)
                },
                _ => {
                    acc
                }

            }
        });

    if a == 1 && b == 2 && c == 3 {
        "Yes"
    } else {
        "No"
    }
}

fn run2(n: &str) -> &'static str {
    let str: String = n.chars().sorted().collect();

    if str == "122333" {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("123233", "Yes"),
            TestCase("123234", "No"),
            TestCase("323132", "Yes"),
            TestCase("500000", "No"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
