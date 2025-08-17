// https://atcoder.jp/contests/abc001/tasks/abc001_4

fn minutes_to_hhmm(mins: usize) -> String {
    let h = mins / 60;
    let m = mins % 60;
    format!("{:02}{:02}", h, m)
}

fn format_ranges(ranges: Vec<(usize, usize)>) -> Vec<String> {
    ranges
        .into_iter()
        .map(|(l, r)| format!("{}-{}", minutes_to_hhmm(l), minutes_to_hhmm(r)))
        .collect()
}

fn run(_n: usize, se: Vec<&str>) -> Vec<String> {
    let ranges: Vec<(usize, usize)> = se
        .into_iter()
        .map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            let l = parts[0].parse::<usize>().unwrap();
            let r = parts[1].parse::<usize>().unwrap();

            let l_total = (l / 100) * 60 + (l % 100);
            let r_total = (r / 100) * 60 + (r % 100);

            let l_rounded = l_total / 5 * 5;
            let r_rounded = ((r_total + 4) / 5 * 5).min(1440);

            (l_rounded, r_rounded)
        })
        .collect();

    let mut imos = vec![0; 1441];

    for (l, r) in ranges {
        imos[l] += 1;
        if r <= 1440 {
            imos[r] -= 1;
        }
    }

    for i in 1..=1440 {
        imos[i] += imos[i - 1];
    }

    let mut ans = Vec::new();
    let mut in_range = false;
    let mut start = 0;

    for i in 0..=1440 {
        if !in_range && imos[i] > 0 {
            in_range = true;
            start = i;
        } else if in_range && imos[i] == 0 {
            in_range = false;
            ans.push((start, i));
        }
    }

    format_ranges(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc001_d() {
        let tests = [
            TestCase(4, vec!["1148-1210", "1323-1401", "1106-1123", "1129-1203"], vec!["1105-1210", "1320-1405"]),
            TestCase(1, vec!["0000-2359"], vec!["0000-2400"]),
            TestCase(6, vec!["1157-1306", "1159-1307", "1158-1259", "1230-1240", "1157-1306", "1315-1317"], vec!["1155-1310", "1315-1320"]),
        ];

        for TestCase(n, se, expected) in tests {
            assert_eq!(run(n, se), expected);
        }
    }
}
