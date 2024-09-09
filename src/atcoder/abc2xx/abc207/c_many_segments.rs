// https://atcoder.jp/contests/abc207/tasks/abc207_c

fn func(t: usize, l: usize, r: usize) -> (f64, f64) {
    match t {
        1 => {
            (l as f64, r as f64)
        },
        2 => {
            (l as f64, r as f64 - 0.1)
        },
        3 => {
            (l as f64 + 0.1, r as f64)
        },
        4 => {
            (l as f64 + 0.1, r as f64 - 0.1)
        },
        _ => unreachable!(),
    }
}

fn run(n: usize, tlr: Vec<(usize, usize, usize)>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            let a = func(tlr[i].0, tlr[i].1, tlr[i].2);
            let b = func(tlr[j].0, tlr[j].1, tlr[j].2);

            if a.1 < b.0 || b.1 < a.0 {
                continue;
            }

            ans += 1;
        }
    }


    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(1, 1, 2), (2, 2, 3), (3, 2, 4)], 2),
            TestCase(19, vec![(4, 210068409, 221208103), (4, 16698200, 910945204), (4, 76268400, 259148324), (4, 370943597, 566244099), (1, 428897569, 509621648), (4, 250946752, 823720940), (1, 642505376, 868415585), (2, 619091266, 868230937), (2, 306543999, 654038916), (4, 486033777, 715789417), (1, 527225177, 583184547), (2, 885292456, 900938600), (3, 264004185, 486613485), (2, 345310564, 818091849), (1, 152544274, 521564294), (4, 13819154, 555218435), (3, 507364086, 545932413), (4, 797872271, 935850550), (2, 415488246, 685203818)], 102),
        ];

        for TestCase(n, tlr, expected) in tests {
            assert_eq!(run(n, tlr), expected);
        }
    }
}
