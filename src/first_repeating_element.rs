// Найти первый повторяющийся элемент
// Условие:
// Дан вектор чисел Vec<i32>. Нужно вернуть первое число, которое повторяется.
// Если таких нет — вернуть None.
//
// Важно: именно первое по порядку повторение в массиве.

#![allow(dead_code)]
use std::collections::HashSet;

fn find_first_repeating_element(nums: &[i32]) -> Option<i32> {
    let mut seen_before = HashSet::new();
    for num in nums {
        if !seen_before.insert(num) {
            return Some(*num);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4, 2, 5];
        let result = find_first_repeating_element(&nums);
        assert_eq!(Some(2), result);
    }
    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_first_repeating_element(&nums);
        assert_eq!(None, result);
    }
    #[test]
    fn test_3() {
        let nums = vec![3, 3, 4, 5, 6];
        let result = find_first_repeating_element(&nums);
        assert_eq!(Some(3), result);
    }
}
