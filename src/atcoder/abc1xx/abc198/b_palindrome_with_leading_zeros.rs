// https://atcoder.jp/contests/abc198/tasks/abc198_b

fn check(s: &str) -> bool {
	s.chars().eq(s.chars().rev())
}

fn run(n: usize) -> &'static str {
	if n == 0 {
		return "Yes";
	}

	let mut num = n;

	// numの末尾0を取り除く
	// (10で割り切れる限り割る)
	while num % 10 == 0 {
		num /= 10
	}

	if check(&num.to_string()) {
		"Yes"
	} else {
		"No"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, &'static str);

	#[test]
	fn abc198_b() {
		let tests = [
			TestCase(1210, "Yes"),
			TestCase(12100000000, "Yes"),
			TestCase(777, "Yes"),
			TestCase(123456789, "No"),
		];

		for TestCase(n, expected) in tests {
			assert_eq!(run(n), expected);
		}
	}
}
