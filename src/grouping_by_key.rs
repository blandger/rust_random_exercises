// Группировка по ключу
// Реализуй функцию:
// fn group_by<T, K, F>(items: Vec<T>, key_selector: F) -> HashMap<K, Vec<T>>
// where
//     K: Eq + Hash,
//     F: Fn(&T) -> K,
//     T: Clone,

// Описание:
// Принимает вектор элементов items: Vec<T>.
//
// Функция key_selector получает ссылку на элемент &T и возвращает ключ K.
//
// Нужно сгруппировать элементы в HashMap<K, Vec<T>>, где ключ — это результат key_selector(item), а значение — вектор элементов, попавших под этот ключ.

use std::collections::HashMap;
use std::hash::Hash;

fn group_by<T, K, F>(items: Vec<T>, key_selector: F) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
    T: Clone,
{
    let mut result = HashMap::new();
    for item in &items {
        let key = key_selector(item);
        result.entry(key).or_insert_with(Vec::new).push(item.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let names = vec!["alice", "bob", "amelia", "brad", "colly", "camala"];
        let grouped = group_by(names, |name| name.chars().next().unwrap());
        println!("{:?}", grouped);

        assert_eq!(grouped[&'a'], vec!["alice", "amelia"]);
        assert_eq!(grouped[&'b'], vec!["bob", "brad"]);
        assert_eq!(grouped[&'c'], vec!["colly", "camala"]);

    }
}
