// https://atcoder.jp/contests/abc033/tasks/abc033_b

fn run(_n: usize, v: Vec<(&str, usize)>) -> String {
    let sum: usize = v.iter()
        .map(|t| t.1)
        .sum();

    v.iter()
        .find(|t| {
            t.1 > sum/2
        })
        .map(|t| {
            t.0
        })
        .unwrap_or("atcoder")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![("unagi", 20), ("usagi", 13), ("snuke", 42), ("smeke", 7)], "snuke"),
            TestCase(5, vec![("a", 10), ("b", 20), ("c", 30), ("d", 40), ("e", 100)], "atcoder"),
            TestCase(14, vec![("yasuzuka", 3340), ("uragawara", 4032), ("oshima", 2249), ("maki", 2614), ("kakizaki", 11484), ("ogata", 10401), ("kubiki", 9746), ("yoshikawa", 5142), ("joetsu", 100000), ("nakago", 4733), ("itakura", 7517), ("kiyosato", 3152), ("sanwa", 6190), ("nadachi", 3169)], "joetsu"),
        ];

        for TestCase(n, v, expected) in tests {
            assert_eq!(run(n, v), expected);
        }
    }
}
