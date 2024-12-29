// https://atcoder.jp/contests/abc062/tasks/abc062_b

fn run(h: usize, w: usize, a: Vec<&str>) -> Vec<String> {
    (0..h+2)
        .enumerate()
        .map(|(i, _)| {
            if i == 0 || i == h+1 {
                "#".to_string().repeat(w+2)
            } else {
                format!("#{}#", &a[i-1])
            }
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);
    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, vec!["abc", "arc"], vec!["#####", "#abc#", "#arc#", "#####"]),
            TestCase(1, 1, vec!["z"], vec!["###", "#z#", "###"]),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}
