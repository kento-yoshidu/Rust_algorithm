// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_A

fn run() -> &'static str {
    "Hello World"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str);

    #[test]
    fn itp1_1_a() {
        assert_eq!(run(), "Hello World");
    }
}
