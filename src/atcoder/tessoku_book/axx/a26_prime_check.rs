// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_z

fn is_prime(num: usize) -> &'static str {
    if (2..=(num as f64).sqrt() as usize)
        .any(|i| num % i == 0) {
            "No"
        } else {
            "Yes"
        }
}

fn run(_q: usize, x: &Vec<usize>) -> Vec<&'static str> {
    x.into_iter()
        .map(|num| {
            is_prime(*num)
        })
        .collect()
}

fn run2(_q: usize, x: &Vec<usize>) -> Vec<&'static str> {
    let max = 300000;
    let sq = (max as f64).sqrt() as usize;

    let mut sieve: Vec<bool> = (1..=max).map(|_| false).collect();

    sieve[0] = true;

    for i in 2..=sq {
        if sieve[i-1] == true {
            continue;
        }

        for j in i..=max/i {
            sieve[(j * i)-1] = true
        }
    }

    x.into_iter()
        .map(|num| {
            if sieve[num-1] == false {
                "Yes"
            } else {
                "No"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn tessoku_a26() {
        let tests = [
            TestCase(4, vec![17, 31, 35, 49], vec!["Yes", "Yes", "No", "No"]),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, &x), expected);
            assert_eq!(run2(n, &x), expected);
        }
    }
}
