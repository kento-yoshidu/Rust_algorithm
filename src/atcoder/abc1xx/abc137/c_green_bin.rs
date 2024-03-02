// https://atcoder.jp/contests/abc137/tasks/abc137_c

use itertools::Itertools;

pub fn run(n: usize, s: Vec<&str>) -> usize {
    let mut vec: Vec<String> = s.iter()
        .map(|str| {
            str.chars()
                .collect::<Vec<char>>()
                .iter()
                .sorted()
                .collect::<String>()
        })
        .collect();

    vec.sort();
    vec.dedup();

    println!("{:?}", vec);
    let ans = n - vec.len();

    /*
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            if vec[i] == vec[j] {
                ans += 1;
            }
        }
    }
    */

    ans * (ans - 1) / 2
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["acornistnt", "peanutbomb", "constraint"], 1),
            TestCase(2, vec!["oneplustwo", "ninemodsix"], 0),
            TestCase(5, vec!["abaaaaaaaa", "oneplustwo", "aaaaaaaaba", "twoplusone", "aaaabaaaaa"], 4),
            TestCase(2, vec!["aaaaaaaaaa", "aaaaaaaaab"], 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
*/
