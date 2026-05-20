// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_D

fn run(t: usize) -> String {
    let mut time = t;

    let h = time / (60*60);

    time -= h*60*60;

    let m = time / 60;

    time -= m*60;

    format!("{}:{}:{}", h, m, time)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn itp1_1_d() {
        let tests = [
            TestCase(46979, "13:2:59"),
        ];

        for TestCase(t, expected) in tests {
            assert_eq!(run(t), expected);
        }
    }
}
