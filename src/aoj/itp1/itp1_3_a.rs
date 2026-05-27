// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/3/ITP1_3_A

fn run() -> Vec<&'static str> {
    vec!["Hello World"; 1000]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>);

    #[test]
    fn itp1_3_a() {
        let tests = [
            TestCase(vec!["Hello World"; 1000]),
        ];

        for TestCase(expected) in tests {
            assert_eq!(run(), expected);
        }
    }
}