// https://atcoder.jp/contests/abc090/tasks/abc090_a

fn run(c: Vec<&str>) -> String {
	let vec: Vec<Vec<char>> = c.iter().map(|s| {
		s.chars().collect()
	}).collect();

	format!("{}{}{}", vec[0][0], vec[1][1], vec[2][2])
}

#[cfg(test)]
mod test {
	use super::*;

	struct TestCase(Vec<&'static str>, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase(vec!["ant", "obe", "rec"], "abc"),
			TestCase(vec!["edu", "cat", "ion"], "ean"),
		];

		for TestCase(c, expected) in tests {
			assert_eq!(run(c), expected);
		}
	}
}
