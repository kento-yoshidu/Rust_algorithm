// https://atcoder.jp/contests/abc198/tasks/abc198_b

fn check(s: &str) -> bool {
	s.chars().eq(s.chars().rev())
}

fn run(n: usize) -> String {
	if n == 0 {
		return String::from("Yes")
	}

	let mut num = n;

	// numの末尾0を取り除く
	// (10で割り切れる限り割る)
	while num % 10 == 0 {
		num /= 10
	}

	if check(&num.to_string()) {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run(1210));
		assert_eq!(String::from("Yes"), run(12100000000));
		assert_eq!(String::from("Yes"), run(777));
		assert_eq!(String::from("No"), run(123456789));
	}
}
