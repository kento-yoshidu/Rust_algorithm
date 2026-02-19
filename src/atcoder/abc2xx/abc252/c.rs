// https://atcoder.jp/contests/abc252/tasks/abc252_c

fn run(n: usize, s: Vec<&str>) -> usize {
    let s: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut time = vec![Vec::new(); 10];

    let mut count = vec![vec![0; 10]; 10];

    for i in 0..n {
        for j in 0..10 {
            let digit = s[i][j].to_digit(10).unwrap() as usize;

            count[digit][j] += 1;

            let t = j + (count[digit][j] - 1) * 10;

            time[digit].push(t);
        }
    }

    time.into_iter()
        .filter(|v| !v.is_empty())
        .map(|v| *v.iter().max().unwrap())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn abc252_c() {
        let tests = [
            TestCase(3, vec!["1937458062", "8124690357", "2385760149"], 6),
            TestCase(5, vec!["0123456789", "0123456789", "0123456789", "0123456789", "0123456789"], 40),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
