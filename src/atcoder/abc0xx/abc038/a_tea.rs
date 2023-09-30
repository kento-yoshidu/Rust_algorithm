// https://atcoder.jp/contests/abc038/tasks/abc038_a

pub fn run(s: String) -> String {
	if s.chars().last().unwrap() == 'T' {
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
		assert_eq!(String::from("Yes"), run(String::from("ICEDT")));
		assert_eq!(String::from("No"), run(String::from("MUGICHA")));
		assert_eq!(String::from("Yes"), run(String::from("OOLONGT")));
		assert_eq!(String::from("Yes"), run(String::from("T")));
		assert_eq!(String::from("No"), run(String::from("TEA")));
	}
}
