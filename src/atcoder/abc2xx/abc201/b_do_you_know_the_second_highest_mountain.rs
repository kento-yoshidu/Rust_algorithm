// https://atcoder.jp/contests/abc201/tasks/abc201_b

fn run(n: usize, a: Vec<(&str, isize)>) -> String {
    let mut vec = a.clone();

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    vec[n - 2].0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, isize)>, &'static str);

    #[test]
    fn abc201_b() {
        let tests = [
            TestCase(3, vec![("Everest", 8849), ("K2", 8611), ("Kangchenjunga", 8586)], "K2"),
            TestCase(4, vec![("Kita", 3193), ("Aino", 3189), ("Fuji", 3776), ("Okuhotaka", 3190)], "Kita"),
            TestCase(4, vec![("QCFium", 2846), ("chokudai", 2992), ("kyoprofriends", 2432), ("penguinman", 2390)], "QCFium", ),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
