// https://atcoder.jp/contests/abc279/tasks/abc279_c

fn run(h: usize, w: usize, s: Vec<&str>, t: Vec<&str>) -> &'static str {
    let s_vec: Vec<Vec<char>> = s.iter().map(|a| a.chars().collect()).collect();
    let t_vec: Vec<Vec<char>> = t.iter().map(|a| a.chars().collect()).collect();

    let mut a = vec![Vec::new(); w];
    let mut b = vec![Vec::new(); w];

    for i in 0..w {
        for j in 0..h {
            a[i].push(s_vec[j][i]);
            b[i].push(t_vec[j][i]);
        }
    }

    a.sort();
    b.sort();

    if a == b {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, vec!["##.#", "##..", "#..."], vec![".###", "..##", "...#"], "Yes"),
            TestCase(3, 3, vec!["#.#", ".#.", "#.#"], vec!["##.", "##.", ".#."], "No"),
            TestCase(2, 1, vec!["#", "."], vec!["#", "."], "Yes"),
            TestCase(8, 7, vec!["#..#..#", ".##.##.", "#..#..#", ".##.##.", "#..#..#", ".##.##.", "#..#..#", ".##.##."], vec![ "....###", "####...", "....###", "####...", "....###", "####...", "....###", "####..."], "Yes"),
        ];

        for TestCase(h, w, s, t, expected) in tests {
            assert_eq!(run(h, w, s, t), expected);
        }
    }
}

// https://programming-hiroba.com/abc279-c/
