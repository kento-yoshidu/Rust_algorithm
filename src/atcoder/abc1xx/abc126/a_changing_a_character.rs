// https://atcoder.jp/contests/abc126/tasks/abc126_a

pub fn run(_n: usize, k: usize, s: String) -> String {
	s.chars().enumerate().map(|(index, c)| {
		if index == k-1 {
			c.to_ascii_lowercase()
		} else {
			c
		}
	}).collect::<String>()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("aBC"), run(3, 1, String::from("ABC")));
		assert_eq!(String::from("CAbA"), run(4, 3, String::from("CABA")));
	}
}
