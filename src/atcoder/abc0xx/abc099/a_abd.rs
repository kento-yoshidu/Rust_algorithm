// https://atcoder.jp/contests/abc099/tasks/abc099_a

fn run(n: usize) -> &'static str {
	if n < 1000 {
		"ABC"
	} else {
		"ABD"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase(999, "ABC"),
			TestCase(1000, "ABD"),
			TestCase(1481, "ABD"),
		];

		for TestCase(n, expected) in tests {
			assert_eq!(run(n), expected);
		}
	}
}
