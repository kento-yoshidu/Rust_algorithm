// https://projecteuler.net/problem=2

pub fn run() -> usize {
    let mut fib: Vec<usize> = Vec::new();

    fib.push(0);
    fib.push(1);

    for i in 2.. {
        fib.push(fib[i-2] + fib[i-1]);

        if fib[i] > 4000000 {
            break
        }
    }

    fib.iter().filter(|&num| {
        num % 2 == 0
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4613732, run());
    }
}
