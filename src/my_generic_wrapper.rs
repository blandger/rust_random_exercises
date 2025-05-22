// Реализовать собственный тип, обобщённый по типу, и использовать его с типажами
// Создай структуру Wrapper<T>, которая оборачивает любое значение типа T, и реализуй функцию max_wrapper, которая сравнивает два таких обёрнутых значения и возвращает большее из них.
//
// ✨ Требования:
// Используй дженерики.
//
// - Используй стандартные типажи (Clone, Ord, PartialEq, и т.д.).
// - Используй стандартные типажи (Clone, Ord, PartialEq, и т.д.).
// - Напиши юнит-тесты.
// - Сделай Wrapper<T> удобным для отладки (#[derive(Debug)]).
#![allow(dead_code)]
use std::cmp::max;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Wrapper<T: Copy> {
    inner: T,
}

impl<T: Copy> Wrapper<T> {
    fn unwrap(self) -> T {
        self.inner
    }
}

fn max_wrapper<T: Ord + Copy>(a: Wrapper<T>, b: Wrapper<T>) -> Wrapper<T> {
    max(a, b)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let a = Wrapper { inner: 10 };
        let b = Wrapper { inner: 20 };
        let max = max_wrapper(a, b);
        assert_eq!(max.unwrap(), 20);
    }
}
