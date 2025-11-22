// https://atcoder.jp/contests/abc222/tasks/abc222_c

fn janken(a: char, b: char) -> isize {
    if a == b {
        -1
    } else if a == 'G' && b == 'P' {
        1
    } else if a == 'C' && b == 'G' {
        1
    } else if a == 'P' && b == 'C' {
        1
    } else {
        0
    }
}

fn run(n: usize, m: usize, a: Vec<&str>) -> Vec<usize> {
    let mut rank = Vec::new();

    for i in 0..2*n {
        rank.push((0, i));
    }

    for i in 0..m {
        for j in 0..n {
            let p1 = rank[2*j].1;
            let p2 = rank[2*j+1].1;

            let result = janken(a[p1].chars().nth(i).unwrap(), a[p2].chars().nth(i).unwrap());

            if result != -1 {
                rank[((2*j) as isize + result) as usize].0 -= 1;
            }
        }

        rank.sort();
    }

    rank.iter()
        .map(|(_, i)| {
            i + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<usize>);

    #[test]
    fn abc222_c() {
        let tests = [
            TestCase(2, 3, vec!["GCP", "PPP", "CCC", "PPC"], vec![3, 1, 2, 4]),
            TestCase(2, 2, vec!["GC", "PG", "CG", "PP"], vec![1, 2, 3, 4])
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
