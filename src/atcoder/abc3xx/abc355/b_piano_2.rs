// https://atcoder.jp/contests/abc355/tasks/abc355_b

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> &'static str {
    let mut vec: Vec<usize> = a.clone();

    vec.append(&mut b.clone());

    vec.sort();

    if vec.windows(2).any(|arr| {
        a.contains(&arr[0]) && a.contains(&arr[1])
    }) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec![3, 2, 5], vec![4, 1], "Yes"),
            TestCase(3, 2, vec![3, 1, 5], vec![4, 2], "No"),
            TestCase(1, 1, vec![1], vec![2], "No"),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
