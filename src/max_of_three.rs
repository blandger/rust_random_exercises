// Найти максимум из трёх значений
// Требуется: Написать функцию max_of_three<T>, которая принимает три значения одного типа T и возвращает наибольшее из них.

// Ограничение: Тип T должен поддерживать сравнение (PartialOrd) и копирование (Copy или Clone, на выбор).
#![allow(dead_code)]
use std::cmp::max;
pub fn max_of_three<T>(one: T, two: T, three: T) -> T
where T: PartialEq + Clone + Ord {
    let first = max(one, two);
    max(first, three)
}
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct TestStruct {
    pub string: String
}
#[cfg(test)]
mod tests {
    use crate::max_of_three::{max_of_three, TestStruct};
    #[test]
    fn test_max_of_three() {
        let one: TestStruct = TestStruct { string: String::from("1") };
        let two: TestStruct = TestStruct { string: String::from("2") };
        let three: TestStruct = TestStruct { string: String::from("3") };
        let result = max_of_three(one, two, three);
        assert_eq!(result, TestStruct { string: String::from("3")});
    }
}
