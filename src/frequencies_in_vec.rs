// Задача: Частота элементов в векторе
// Условие:
// 
// Напиши функцию, которая получает на вход срез &[T], и возвращает HashMap<T, usize>, где T — тип, поддерживающий хэш и сравнение, а значение — количество вхождений каждого элемента.

// Требования:
// Обобщи функцию по типу T, с T: Eq + Hash.
// 
// Тип возвращаемого значения: HashMap<T, usize>.
// 
// Решение не должно требовать Clone у T.
// 
// Подумай: нужно ли использовать ссылки? (ДА !)
#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

pub fn frequencies<T: Eq + Hash>(slice: &[T]) -> HashMap<&T, usize> {
    let mut map = HashMap::new();
    slice.into_iter().for_each(|item| {
        *map.entry(item).or_insert(0) += 1;
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frequencies() {
        let input = vec!["apple", "banana", "apple", "orange", "banana", "apple"];
        let result = frequencies(&input);
        assert_eq!(result[&"apple"], 3);
        assert_eq!(result[&"banana"], 2);
        assert_eq!(result[&"orange"], 1);
    }
}