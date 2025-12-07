// https://atcoder.jp/contests/abc239/tasks/abc239_d

use num_integer::Roots;

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    let mut eratosthenes = vec![true; 201];
    eratosthenes[0] = false;
    eratosthenes[1] = false;

    for i in 2..=200.sqrt() {
        if eratosthenes[i] == true {
            for j in (i*i..=200).step_by(i) {
                eratosthenes[j] = false;
            }
        }
    }

    for i in a..=b {
        let mut flag = true;

        for j in c..=d {
            if eratosthenes[i+j] == true {
                flag = false;
            }
        }

        if flag {
            return "Takahashi";
        }
    }

    "Aoki"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn abc239_d() {
        let tests = [
            TestCase(2, 3, 3, 4, "Aoki"),
            TestCase(1, 100, 50, 60, "Takahashi"),
            TestCase(3, 14, 1, 5, "Aoki"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
