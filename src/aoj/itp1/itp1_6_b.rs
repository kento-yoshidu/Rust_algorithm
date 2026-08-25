// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/6/ITP1_6_B

fn run(_n: usize, t: Vec<(char, usize)>) -> Vec<(char, usize)> {
    let suits = ['S', 'H', 'C', 'D'];
    let mut present = [[false; 13]; 4];

    for (suit, n) in t {
        let i = suits.iter().position(|&s| s == suit).unwrap();
        present[i][n - 1] = true;
    }

    suits.iter().enumerate()
        .flat_map(|(i, &suit)| {
            present[i].iter().enumerate()
                .filter(|(_, &b)| !b)
                .map(move |(j, _)| (suit, j + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(char, usize)>, Vec<(char, usize)>);

    #[test]
    fn itp1_6_a() {
        let tests = [
            TestCase(47, vec![('S', 10), ('S', 11), ('S', 12), ('S', 13), ('H', 1), ('H', 2), ('S', 6), ('S', 7), ('S', 8), ('S', 9), ('H', 6), ('H', 8), ('H', 9), ('H', 10), ('H', 11), ('H', 4), ('H', 5), ('S', 2), ('S', 3), ('S', 4), ('S', 5), ('H', 12), ('H', 13), ('C', 1), ('C', 2), ('D', 1), ('D', 2), ('D', 3), ('D', 4), ('D', 5), ('D', 6), ('D', 7), ('C', 3), ('C', 4), ('C', 5), ('C', 6), ('C', 7), ('C', 8), ('C', 9), ('C', 10), ('C', 11), ('C', 13), ('D', 9), ('D', 10), ('D', 11), ('D', 12), ('D', 13)], vec![('S', 1), ('H', 3), ('H', 7), ('C', 12), ('D', 8)]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
