// https://atcoder.jp/contests/past201912-open/tasks/past201912_b

fn run(_n: usize, a: Vec<usize>) -> Vec<(&'static str, Option<usize>)> {
    a.windows(2)
        .map(|arr| {
            if arr[0] == arr[1] {
                ("stay", None)
            } else if arr[0] > arr[1] {
                ("down", Some(arr[0] - arr[1]))
            } else {
                ("up", Some(arr[1] - arr[0]))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<(&'static str, Option<usize>)>);

    #[test]
    fn past201912_b() {
        let tests = [
            TestCase(10, vec![9, 10, 3, 100, 100, 90, 80, 10, 30, 10], vec![("up", Some(1)), ("down", Some(7)), ("up", Some(97)), ("stay", None), ("down", Some(10)), ("down", Some(10)), ("down", Some(70)), ("up", Some(20)), ("down", Some(20))]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
