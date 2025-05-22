// Реализуй функцию:
// 
// pub fn most_frequent<T: Eq + Hash>(items: &[T]) -> Option<&T>

// Функция должна вернуть ссылку на элемент, который встречается чаще всего в срезе. Если срез пустой — вернуть None.
// Не используй Clone, всё должно работать на ссылках. 

use std::collections::HashMap;
use std::hash::Hash;

pub fn most_frequent<T: Eq + Hash>(items: &[T]) -> Option<&T> {
    let mut map = HashMap::new();
    items.into_iter().for_each(|item| {
        *map.entry(item).or_insert(0) += 1;
    });
    map.values().max().and_then(|max_value| items.iter().find(|hash_key| map.get(hash_key) == Some(max_value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequent() {
        let items = vec!["apple", "banana", "apple", "orange", "banana", "apple"];
        assert_eq!(most_frequent(&items), Some(&"apple"));

        let items = vec!["a", "b", "b", "a"];
        let result = most_frequent(&items);
        assert!(result == Some(&"a") || result == Some(&"b")); // допустимы оба ответа

        let empty: Vec<i32> = vec![];
        assert_eq!(most_frequent(&empty), None);
    }
}