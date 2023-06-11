use std::collections::HashMap;

#[allow(dead_code)]
pub fn run() -> HashMap<String, usize> {
    let text = String::from("Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.");

    let tmp: Vec<_> = text.split(" ").collect();

    let mut map: HashMap<String, usize> = HashMap::new();

    for (i, word) in tmp.iter().enumerate() {
        match i + 1 {
            1 | 5 | 6 | 7 | 8 | 9 | 15 | 16 | 19 => map.insert(word.chars().nth(0).unwrap().to_string(), i + 1),
            _ => map.insert(word[0..=1].to_string(), i + 1),
        };
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans: HashMap<String, usize> = HashMap::from([
            ("H".to_string(), 1),
            ("He".to_string(), 2),
            ("Li".to_string(), 3),
            ("Be".to_string(), 4),
            ("B".to_string(), 5),
            ("C".to_string(), 6),
            ("N".to_string(), 7),
            ("O".to_string(), 8),
            ("F".to_string(), 9),
            ("Ne".to_string(), 10),
            ("Na".to_string(), 11),
            ("Mi".to_string(), 12),
            ("Al".to_string(), 13),
            ("Si".to_string(), 14),
            ("P".to_string(), 15),
            ("S".to_string(), 16),
            ("Cl".to_string(), 17),
            ("Ar".to_string(), 18),
            ("K".to_string(), 19),
            ("Ca".to_string(), 20)
        ]);

        assert_eq!(ans, run())
    }
}
