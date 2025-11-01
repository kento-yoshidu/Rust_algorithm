// https://atcoder.jp/contests/abc134/tasks/abc134_a

fn run(r: usize) -> usize {
	3 * r * r
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize);

	#[test]
	fn abc134_a() {
		let tests = [
			TestCase(4, 48),
			TestCase(15, 675),
			TestCase(80, 19200),
		];

		for TestCase(r, expected) in tests {
			assert_eq!(run(r), expected);
		}
	}
}
