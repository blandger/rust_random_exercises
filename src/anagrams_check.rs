#![allow(dead_code)]
// Проверка анаграмм
// Условие:
// Даны две строки s1 и s2. Нужно проверить, являются ли они анаграммами.
//
// Анаграммы — это слова, составленные из одних и тех же букв в любом порядке.
// Регистр и пробелы игнорируются.

use std::collections::HashMap;

pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    normalized_counts(s1) == normalized_counts(s2)
}
fn normalized_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in s.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_ascii_lowercase()) {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let result = are_anagrams("listen", "silent");
        assert_eq!(result, true);
    }
    #[test]
    fn test_2() {
        let result = are_anagrams("rail safety", "fairy tales");
        assert_eq!(result, true);
    }
    #[test]
    fn test_3() {
        let result = are_anagrams("hello", "world");
        assert_eq!(result, false);
    }
    #[test]
    fn test_repeated_letters() {
        assert!(!are_anagrams("aabb", "ab"));
    }
    #[test]
    fn test_case_and_space() {
        assert!(are_anagrams("Dormitory", "Dirty room"));
    }
}
