// https://paiza.jp/works/mondai/s_rank_skillcheck_sample/mod7

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut count = [0; 7];

    for n in a {
        count[n % 7] += 1;
    }

    let mut ans = 0;

    for i in 0..7 {
        for j in i..7 {
            for k in j..7 {
                if (i + j + k) % 7 != 0 {
                    continue;
                }

                if i == j && j == k {
                    if count[i] >= 3 {
                        ans += count[i] * (count[i]-1) * (count[i]-2) / 6;
                    }
                } else if i == j {
                    if count[i] >= 2 {
                        ans += count[i] * (count[i] - 1) / 2 * count[k];
                    }
                } else if j == k {
                    if count[j] >= 2 {
                        ans += count[j] * (count[j]-1) / 2 * count[i];
                    }
                } else if i == k {
                    if count[i] >= 2 {
                        ans += count[i] * (count[i]-1) / 2 * count[j];
                    }
                } else {
                    ans += count[i] * count[j] * count[k];
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn paiza_s_skill_check_mihon_problem02() {
        let tests = [
            TestCase(3, vec![10, 4, 14], 1),
            TestCase(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 17),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
