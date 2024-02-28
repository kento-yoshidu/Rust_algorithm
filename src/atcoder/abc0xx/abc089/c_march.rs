// https://atcoder.jp/contests/abc089/tasks/abc089_c

fn run(_n: usize, pcf: Vec<&str>) -> usize {
    let mut vec: Vec<&&str> = pcf.iter()
        .filter(|str| {
            let c = str.chars().nth(0).unwrap();

            c == 'M' || c == 'A' || c == 'R' || c == 'C' || c == 'H'
        })
        .collect();

    vec.sort();

    let mut ans = 0;

    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            for k in j+1..vec.len() {
                let a = vec[i].chars().nth(0).unwrap();
                let b = vec[j].chars().nth(0).unwrap();
                let c = vec[k].chars().nth(0).unwrap();

                if a != b && b != c {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec!["MASHIKE", "RUMOI", "OBIRA", "HABORO", "HOROKANAI"], 2),
            TestCase(4, vec!["ZZ", "ZZZ", "Z", "ZZZZZZZZZZ"], 0),
            TestCase(5, vec!["CHOKUDAI", "RNG", "MAKOTO", "AOKI", "RINGO"], 7),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
