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

	#[test]
	fn test() {
		assert_eq!(String::from("abc"), run(vec!["ant", "obe", "rec"]));
		assert_eq!(String::from("ean"), run(vec!["edu", "cat", "ion"]));
	}
}
