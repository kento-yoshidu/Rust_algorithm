// https://projecteuler.net/problem=3

use num_integer::sqrt;

pub fn run() -> usize {
    let mut num = 600851475143;

    for i in 2..sqrt(600851475143 as usize) {
        if num % i == 0 {
            if i == num {
                return num
            }

            num /= i;

            while num % i == 0 {
                if i == num {
                    return num
                }

                num /= i;
            }
        }
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6857, run());
    }
}
