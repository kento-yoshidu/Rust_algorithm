// https://atcoder.jp/contests/abc092/tasks/abc092_a

fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
	a.min(b) + c.min(d)
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(600, 300, 220, 420, 520),
			TestCase(555, 555, 400, 200, 755),
			TestCase(549, 817, 715, 603, 1152),
		];

		for TestCase(a, b, c, d, expected) in tests {
			assert_eq!(run(a, b, c, d), expected);
		}
	}
}
