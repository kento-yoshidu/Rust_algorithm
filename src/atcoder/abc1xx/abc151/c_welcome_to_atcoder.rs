// https://atcoder.jp/contests/abc151/tasks/abc151_c

pub fn run(n: usize, _m: usize, ps: Option<Vec<(usize, &str)>>) -> (usize, usize) {
    if ps == None {
        return (0, 0)
    }

    let mut ac = vec![false; n];
    let mut wa_tmp = vec![0; n];
    let mut wa = vec![0; n];

    for (p, s) in ps.unwrap().iter() {
        if s == &"AC" {
            if ac[p-1] == false {
                ac[p-1] = true;
            }

            wa[p-1] = wa_tmp[p-1];
        } else {
            if ac[p-1] == false {
                wa_tmp[p-1] += 1;
            }
        }
    }

    let a = ac.into_iter().filter(|b| *b == true).count();
    let w: usize = wa.into_iter().sum();

    (a, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, &'static str)>>, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 5, Some(vec![(1, "WA"), (1, "AC"), (2, "WA"), (2, "AC"), (2, "WA")]), (2, 2)),
            TestCase(100000, 3, Some(vec![(7777, "AC"), (7777, "AC"), (7777, "AC")]), (1, 0)),
            TestCase(6, 0, None, (0, 0)),
            TestCase(1, 5, Some(vec![(1, "WA")]), (0, 0)),
        ];

        for TestCase(n, m, ps, expected) in tests {
            assert_eq!(run(n, m, ps), expected);
        }
    }
}
