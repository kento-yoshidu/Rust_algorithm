// https://yukicoder.me/problems/no/2714

use itertools::Itertools;

fn run(_n: usize, s: Vec<Vec<&str>>) -> usize {
    s.into_iter()
        .filter(|v| {
            v.iter().cloned().sorted().collect::<Vec<_>>() == vec!["akai", "marui", "okii", "umai"]
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<&'static str>>, usize);

    #[test]
    fn yuki_2714() {
        let tests = [
            TestCase(4, vec![vec!["akai", "marui", "okii", "umai"], vec!["okii", "marui", "umai", "akai"], vec!["amai", "marui", "oishi", "umai"], vec!["umai", "umai", "umai", "umai)"]], 2),
            TestCase(2, vec![vec!["akai", "marui", "ookii", "umai"], vec!["akai", "marui", "oki", "umai"]], 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
