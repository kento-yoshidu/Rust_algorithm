// https://atcoder.jp/contests/abc109/tasks/abc109_a

fn run(a: usize, b: usize) -> &'static str {
	if a * b % 2 == 0 {
		"No"
	} else {
		"Yes"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, &'static str);

	#[test]
	fn abc109_a() {
		let tests = [
			TestCase(3, 1, "Yes"),
			TestCase(1, 2, "No"),
			TestCase(2, 2, "No"),
		];

		for TestCase(a, b, expected) in tests {
			assert_eq!(run(a, b), expected);
		}
	}
}
