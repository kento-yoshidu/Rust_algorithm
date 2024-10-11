// https://atcoder.jp/contests/abc039/tasks/abc039_a

fn run(a: usize, b: usize, c: usize) -> usize {
	2 * (a*b + a*c + b*c)
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize, usize);

	#[test]
	fn test() {
		let tests = [
			TestCase(2, 3, 4, 52),
			TestCase(3, 4, 2, 52),
			TestCase(100, 100, 100, 60000),
		];

		for TestCase(a, b, c, expected) in tests {
			assert_eq!(run(a, b, c), expected);
		}
	}
}
