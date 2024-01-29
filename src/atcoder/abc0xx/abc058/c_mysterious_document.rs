// https://atcoder.jp/contests/abc058/tasks/arc071_a

pub fn run(n: usize, s: Vec<&str>) -> String {
    let mut count = vec![vec![0; 26]; n];

    for i in 0..n {
        for c in s[i].chars() {
            count[i][c as usize - 'a' as usize] += 1;
        }
    }

    let mut temp = vec![100; 26];

    for i in 0..26 {
        for j in 0..n {
            temp[i] = temp[i].min(count[j][i]);
        }
    }

    let mut ans = String::new();

    for i in 0..26 {
        if temp[i] != 0 {
            let ci = i as u8 + ('a' as u8);
            let ci = ci as char;

            for _ in 0..temp[i] {
                ans.push(ci);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["cbaa", "daacc", "acacac"], "aac"),
            TestCase(3, vec!["a", "aa", "b"], ""),
            TestCase(3, vec!["abc", "abc", "abc"], "abc"),
            TestCase(6, vec!["rrezfbtuxixjkipxtncfquqdtaniwlevdyodliiwuvbogqzwwp", "dpkdbqvruegiawlwtnivixteu", "nebitituqvdg", "bvedtt", "dbt", "t"], "t"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
