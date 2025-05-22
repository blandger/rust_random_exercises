// Remove Duplicates from Sorted Vector

// На вход даётся отсортированный Vec<i32>.
//
// Нужно удалить дубликаты на месте и вернуть новое количество уникальных элементов.
//
// Функция должна изменять nums напрямую — оставить только уникальные элементы в начале вектора, а остальные можно удалить (обрезать Vec).

// Правила:
// Без дополнительного вектора.
// 
// Без .dedup() — реализовать руками.
// 
// Важно работать с &mut Vec, а не брать Vec<T> по значению.

use std::collections::HashSet;
use std::hash::Hash;

fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut write_index = 1; // Начинаем с 1, так как первый элемент всегда уникален
    for read_index in 1..nums.len() {
        if nums[read_index] != nums[write_index - 1] {
            nums[write_index] = nums[read_index];
            write_index += 1;
        }
    }
    nums.truncate(write_index);
    nums.len()
}

fn remove_duplicates_generic<T, I>(iter: I) -> Vec<T>
where
    T: Eq + Hash + Copy,
    I: IntoIterator<Item = T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for item in iter {
        if seen.insert(item) {
            result.push(item);
        }
    }
    result
}

fn dedup_by_ref<T>(items: &[T]) -> Vec<&T>
where
    T: Eq + Hash {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for item in items {
        if seen.insert(item) {
            result.push(item);
        }
    }
    result
}

fn dedup_by_key<T, K, F>(items: &[T], key_fn: F) -> Vec<&T>
where
    K: Eq + Hash,
    F: Fn(&T) -> K {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for item in items {
        let key = key_fn(item);
        if seen.insert(key) {
            result.push(item);
        }
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_by_key() {
        let input = vec!["apple", "apricot", "banana", "blueberry", "avocado"];
        let result = dedup_by_key(&input, |s| s.chars().next().unwrap());
        let expected = vec![&"apple", &"banana"]; // по первой букве: 'a', 'b'
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dedup_by_ref() {
        let input = vec![3, 1, 2, 1, 2, 1, 2, 3];
        let result = dedup_by_ref(&input);
        let expected = vec![&3, &1, &2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_duplicates_with_integers() {
        let input = vec![3, 1, 2, 1, 2, 1, 2, 3];
        let result = remove_duplicates_generic(input);
        assert_eq!(result, vec![3, 1, 2]);
    }

    #[test]
    fn test_remove_duplicates_with_strings() {
        let input = vec!["a", "b", "a", "c", "b"];
        let result = remove_duplicates_generic(input);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_remove_duplicates_all_unique() {
        let input = vec![10, 20, 30];
        let result = remove_duplicates_generic(input);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_remove_duplicates_empty() {
        let input: Vec<i32> = vec![];
        let result = remove_duplicates_generic(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_duplicates_from_array() {
        let input = [1, 2, 2, 3, 1];
        let result = remove_duplicates_generic(input);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_one() {
        let mut nums = vec![1, 1, 2, 2, 3];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 3);
        assert_eq!(&nums[..len], &[1, 2, 3]);
    }
}
