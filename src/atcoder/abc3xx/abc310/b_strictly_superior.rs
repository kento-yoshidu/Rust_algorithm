// https://atcoder.jp/contests/abc310/tasks/abc310_b

use std::collections::HashSet;

fn check(vec_i: &Vec<usize>, vec_j: &Vec<usize>) -> bool {
    let set_j: HashSet<_> = vec_j.iter().collect();

    vec_i.iter().all(|i| set_j.contains(i))
}

fn check_2(vec_i: &Vec<usize>, vec_j: &Vec<usize>) -> bool {
    let set_i: HashSet<_> = vec_i.iter().collect();

    vec_j.iter().any(|j| !set_i.contains(j))
}

fn run(n: usize, _m: usize, pcf: Vec<(usize, usize, Vec<usize>)>) -> &'static str {
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if pcf[j].0 > pcf[i].0 {
                continue;
            }

            if !check(&pcf[i].2, &pcf[j].2) {
                continue;
            }


            if pcf[j].0 == pcf[i].0 {
                if !check_2(&pcf[i].2, &pcf[j].2) {
                    continue;
                }
            }

            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, Vec<usize>)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 6, vec![(10000, 2, vec![1, 3]), (15000, 3, vec![1, 2, 4]), (30000, 3, vec![1, 3, 5]), (35000, 2, vec![1, 5]), (100000, 6, vec![1, 2, 3, 4, 5, 6])], "Yes"),
            TestCase(4, 4, vec![(3, 1, vec![1]), (3, 1, vec![2]), (3, 1, vec![2]), (4, 2, vec![2, 3])], "No"),
            TestCase(20, 10, vec![(72036, 3, vec![3, 4, 9]), (7716, 4, vec![1, 2, 3, 6]), (54093, 5, vec![1, 6, 7, 8, 10]), (25517, 7, vec![3, 4, 5, 6, 7, 9, 10]), (96930, 8, vec![2, 3, 4, 6, 7, 8, 9, 10]), (47774, 6, vec![2, 4, 5, 6, 7, 9]), (36959, 5, vec![1, 3, 4, 5, 8]), (46622, 7, vec![1, 2, 3, 5, 6, 8, 10]), (34315, 9, vec![1, 3, 4, 5, 6, 7, 8, 9, 10]), (54129, 7, vec![1, 3, 4, 6, 7, 8, 9]), (4274, 5, vec![2, 4, 7, 9, 10]), (16578, 5, vec![2, 3, 6, 7, 9]), (61809, 4, vec![1, 2, 4, 5]), (1659, 5, vec![3, 5, 6, 9, 10]), (59183, 5, vec![1, 2, 3, 4, 9]), (22186, 4, vec![3, 5, 6, 8]), (98282, 4, vec![1, 4, 7, 10]), (72865, 8, vec![1, 2, 3, 4, 6, 8, 9, 10]), (33796, 6, vec![1, 3, 5, 7, 9, 10]), (74670, 4, vec![1, 2, 6, 8])], "Yes"),
        ];

        for TestCase(n, m, pcf, expected) in tests {
            assert_eq!(run(n, m, pcf), expected);
        }
    }
}
