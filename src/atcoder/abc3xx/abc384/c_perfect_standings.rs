// https://atcoder.jp/contests/abc384/tasks/abc384_c

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize, d: usize, e: usize) -> Vec<String> {
    let mut ans = Vec::new();

    let arr = [('A', a), ('B', b), ('C', c), ('D', d), ('E', e)];

    for bit in 1..(1 << 5) {
        let mut str = String::new();
        let mut score = 0;

        for i in 0..5 {
            if bit & (1 << i) > 0 {
                str.push(arr[i].0);
                score += arr[i].1;
            }
        }

        ans.push((str, score));
    }

    ans.into_iter()
        .sorted_by(|a, b| b.1.cmp(&(a.1)).then_with(|| a.0.cmp(&(b.0))))
        .map(|(str, _)| str)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(400, 500, 600, 700, 800, vec!["ABCDE", "BCDE", "ACDE", "ABDE", "ABCE", "ABCD", "CDE", "BDE", "ADE", "BCE", "ACE", "BCD", "ABE", "ACD", "ABD", "ABC", "DE", "CE", "BE", "CD", "AE", "BD", "AD", "BC", "AC", "AB", "E", "D", "C", "B", "A"]),
            TestCase(800, 800, 900, 900, 1000, vec!["ABCDE", "ACDE", "BCDE", "ABCE", "ABDE", "ABCD", "CDE", "ACE", "ADE", "BCE", "BDE", "ABE", "ACD", "BCD", "ABC", "ABD", "CE", "DE", "AE", "BE", "CD", "AC", "AD", "BC", "BD", "AB", "E", "C", "D", "A", "B"]),
            TestCase(128, 256, 512, 1024, 2048, vec!["ABCDE", "BCDE", "ACDE", "CDE", "ABDE", "BDE", "ADE", "DE", "ABCE", "BCE", "ACE", "CE", "ABE", "BE", "AE", "E", "ABCD", "BCD", "ACD", "CD", "ABD", "BD", "AD", "D", "ABC", "BC", "AC", "C", "AB", "B", "A"]),
        ];

        for TestCase(a, b, c, d, e, expected) in tests {
            assert_eq!(run(a, b, c, d, e), expected);
        }
    }
}
