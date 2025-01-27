// https://atcoder.jp/contests/abc082/tasks/abc082_a

fn run(a: usize, b: usize) -> usize {
	((a as f64 + b as f64) / 2.0).ceil() as usize
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(1, 3, 2),
			TestCase(7, 4, 6),
			TestCase(5, 5, 5),
		];

		for TestCase(a, b, expected) in tests {
			assert_eq!(run(a, b), expected);
		}
	}
}
