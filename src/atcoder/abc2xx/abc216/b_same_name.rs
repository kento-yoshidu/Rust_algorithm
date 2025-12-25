// https://atcoder.jp/contests/abc216/tasks/abc216_b

use itertools::Itertools;

fn run(_n: usize, v: Vec<(&str, &str)>) -> &'static str {
    if v.iter().permutations(2).any(|arr| {
        arr[0].0 == arr[1].0 && arr[0].1 == arr[1].1
    }) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, &'static str)>, &'static str);

    #[test]
    fn abc216_b() {
        let tests = [
            TestCase(3, vec![("tanaka", "taro"), ("sato", "hanako"), ("tanaka", "taro")], "Yes"),
            TestCase(3, vec![("saito", "ichiro"), ("saito", "jiro"), ("saito", "saburo")], "No"),
            TestCase(4, vec![("sypdgido", "bkseq"), ("bajsqz", "hh"), ("ozjekw", "mcybmtt"), ("qfeysvw", "dbo")], "No"),
        ];

        for TestCase(n, v, expected) in tests {
            assert_eq!(run(n, v), expected);
        }
    }
}
