// https://atcoder.jp/contests/abc098/tasks/abc098_a

fn run(a: isize, b: isize) -> isize {
	[a + b, a - b, a * b].into_iter().max().unwrap()
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(isize, isize, isize);

	#[test]
	fn test() {
		let tests = [
			TestCase(3, 1, 4),
			TestCase(4, -2, 6),
			TestCase(0, 0, 0),
		];

		for TestCase(a, b, expected) in tests {
			assert_eq!(run(a, b), expected);
		}
	}
}
