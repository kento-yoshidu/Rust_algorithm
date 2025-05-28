// https://atcoder.jp/contests/abc084/tasks/abc084_a

fn run(m: usize) -> usize {
	(24 - m) + 24
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(21, 27),
			TestCase(12, 36),
		];

		for TestCase(m, expected) in tests {
			assert_eq!(run(m), expected);
		}
	}
}
