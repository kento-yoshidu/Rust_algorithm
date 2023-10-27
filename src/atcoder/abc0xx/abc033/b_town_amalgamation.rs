// https://atcoder.jp/contests/abc033/tasks/abc033_b


pub fn run(_n: usize, v: Vec<(&str, usize)>) -> String {
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

    #[test]
    fn test() {
        assert_eq!(String::from("snuke"), run(4, vec![("unagi", 20), ("usagi", 13), ("snuke", 42), ("smeke", 7)]));
        assert_eq!(String::from("atcoder"), run(5, vec![("a", 10), ("b", 20), ("c", 30), ("d", 40), ("e", 100)]));
        assert_eq!(String::from("joetsu"), run(14, vec![("yasuzuka", 3340), ("uragawara", 4032), ("oshima", 2249), ("maki", 2614), ("kakizaki", 11484), ("ogata", 10401), ("kubiki", 9746), ("yoshikawa", 5142), ("joetsu", 100000), ("nakago", 4733), ("itakura", 7517), ("kiyosato", 3152), ("sanwa", 6190), ("nadachi", 3169)]));
    }
}
