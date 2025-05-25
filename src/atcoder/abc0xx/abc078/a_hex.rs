// https://atcoder.jp/contests/abc078/tasks/abc078_a

fn run(x: char, y: char) -> char {
	let a = x as usize;
	let b = y as usize;

	if a < b {
		'<'
	} else if a == b {
		'='
	} else {
		'>'
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(char, char, char);

	#[test]
	fn test() {
		let tests = [
			TestCase('A', 'B', '<'),
			TestCase('E', 'C', '>'),
			TestCase('F', 'F', '='),
		];

		for TestCase(x, y, expected) in tests {
			assert_eq!(run(x, y), expected);
		}
	}
}
