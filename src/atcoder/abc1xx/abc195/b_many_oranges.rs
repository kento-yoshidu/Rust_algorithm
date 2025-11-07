// https://atcoder.jp/contests/abc195/tasks/abc195_b

fn run(a: usize, b: usize, w: usize) -> String {
    let mut min = std::usize::MAX;
    let mut max = 0;

    for i in 1..=1000*1000 {
        if a*i <= w*1000 && 1000*w <= b*i {
            min = min.min(i);
            max = max.max(i);
        }
    }

    if max == 0 {
        "UNSATISFIABLE".to_string()
    } else {
        format!("{} {}", min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc195_b() {
        let tests = [
            TestCase(100, 200, 2, "10 20"),
            TestCase(120, 150, 2, "14 16"),
            TestCase(300, 333, 1, "UNSATISFIABLE"),
        ];

        for TestCase(a, b, w, expected) in tests {
            assert_eq!(run(a, b, w), expected);
        }
    }
}
