// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_z

fn is_prime(num: usize) -> String {
    if (2..=(num as f64).sqrt() as usize)
        .any(|i| num % i == 0) {
            String::from("No")
        } else {
            String::from("Yes")
        }
}

pub fn run(_q: usize, x: Vec<usize>) -> Vec<String> {
    x.iter()
        .map(|num| {
            is_prime(*num)
        })
        .collect()
}

pub fn run2(_q: usize, x: Vec<usize>) -> Vec<String> {
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

    x.iter()
        .map(|num| {
            if sieve[*num-1] == false {
                String::from("Yes")
            } else {
                String::from("No")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("Yes"), String::from("Yes"), String::from("No"), String::from("No")], run(4, vec![17, 31, 35, 49]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![String::from("Yes"), String::from("Yes"), String::from("No"), String::from("No")], run2(4, vec![17, 31, 35, 49]));
    }
}
