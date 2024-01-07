pub fn seive_of_eratosthenes(x: Vec<usize>) -> Vec<bool> {
    let max = x.iter().max().unwrap();
    let sq = (*max as f64).sqrt() as usize;

    let mut sieve: Vec<bool> = (1..=*max).map(|_| true).collect();

    sieve[0] = false;

    for i in 2..=sq {
        if sieve[i-1] == false {
            continue;
        }

        for j in i..=max/i {
            sieve[(j * i)-1] = false
        }
    }

    x.iter()
        .map(|num| {
            sieve[*num-1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![true, true, false, true, false, true, false, false, false], seive_of_eratosthenes(vec![2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true], seive_of_eratosthenes(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]));
    }
}
