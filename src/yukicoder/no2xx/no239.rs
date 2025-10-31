// https://yukicoder.me/problems/no/239

fn run(n: usize, a: Vec<Vec<&str>>) -> isize {
    let mut ans = Vec::new();

    'outer: for i in 0..n {
        for j in 0..n {
            if a[j][i] != "nyanpass" && a[j][i] != "-" {
                continue 'outer;
            }
        }

        ans.push(i+1);
    }

    if ans.len() == 1 {
        ans[0] as isize
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<&'static str>>, isize);

    #[test]
    fn yuki_239() {
        let tests = [
            TestCase(4, vec![vec!["-", "nyanpass", "uissu", "ohayo"], vec!["konbanwa", "-", "ohayo", "ohayo"], vec!["ohayogozaimasu", "nyanpass", "-", "komachanyuuna"], vec!["konnichiwa", "nyanpass", "komachanohayo", "-"]], 2),
            TestCase(3, vec![vec!["-", "nyanpass", "ohayo"], vec!["nyanpass", "-", "chicchakunaiyo"], vec!["nyanpass", "nyanpass", "-"]], -1),
            TestCase(3, vec![vec!["-", "ohayo", "nyanpass"], vec!["nyanpass", "-", "ohayo"], vec!["nyanpass", "ohayo", "-"]], 1),
            TestCase(2, vec![vec!["-", "ohayo"], vec!["zukoo", "-"]], -1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
