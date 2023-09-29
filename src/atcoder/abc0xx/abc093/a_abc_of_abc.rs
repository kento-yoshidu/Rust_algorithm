// https://atcoder.jp/contests/abc093/tasks/abc093_a

fn run(s: String) -> String {
	let mut chars: Vec<char> = s.chars().collect();

	chars.sort();

	if String::from("abc") == chars.iter().collect::<String>() {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run(String::from("bac")));
		assert_eq!(String::from("No"), run(String::from("bab")));
		assert_eq!(String::from("Yes"), run(String::from("abc")));
		assert_eq!(String::from("No"), run(String::from("aaa")));
	}
}
