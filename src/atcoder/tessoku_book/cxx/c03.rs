// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fa

fn run(d: usize, x: isize, a: Vec<isize>, _q: usize, st: Vec<(usize, usize)>) -> Vec<String> {
    let mut acc = Vec::new();

    acc.push(x);

    for i in 1..d {
        acc.push(acc[i-1] + a[i-1]);
    }

    st.into_iter()
        .map(|(s, t)| {
            if acc[s-1] > acc[t-1] {
                s.to_string()
            } else if acc[s-1] < acc[t-1] {
                t.to_string()
            } else {
                "Same".to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, usize, Vec<(usize, usize)>, Vec<&'static str>);

    #[test]
    fn tessoku_c03() {
        let tests = [
            TestCase(5, 30, vec![-10, 20, -10, 20], 3, vec![(1, 2), (3, 5), (1, 4)], vec!["1", "5", "Same"]),
        ];

        for TestCase(d, x, a, q, st, expected) in tests {
            assert_eq!(run(d, x, a, q, st), expected);
        }
    }
}
