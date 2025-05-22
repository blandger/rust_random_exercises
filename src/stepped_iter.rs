// Реализовать свой итератор
// Создайте итератор, который будет возвращать элементы из вектора, но с определенным шагом. Например, если шаг равен 2, то итератор будет возвращать каждый второй элемент вектора.

// Ожидаемый результат:
// Реализуйте структуру, которая будет использовать Rust-итератор.
// 
// Итератор должен быть настроен на произвольный шаг.
// 
// Напишите тесты, чтобы убедиться, что итератор работает корректно.
// 
// Подсказка:
// 
// Вам нужно реализовать структуру, которая будет хранить данные, и создать для неё итератор. Вам нужно будет реализовать следующие методы для итератора: next() и size_hint(), чтобы итератор знал свой размер.

#[derive(Debug)]
struct StepIterator<'a, T> {
    data: &'a [T],
    step: usize,
    current: usize,
}

impl<'a, T> StepIterator<'a, T> {
    fn new(data: &'a [T], step: usize) -> Self {
        StepIterator {
            data,
            step,
            current: 0,
        }
    }
}

impl<'a, T> Iterator for StepIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = &self.data[self.current];
            self.current += self.step;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_iterator() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let iter = StepIterator::new(&data, 2); // Шаг 2

        let result: Vec<_> = iter.collect();
        assert_eq!(result, vec![&1, &3, &5, &7, &9]);
    }

    #[test]
    fn test_step_iterator_with_step_3() {
        let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        let iter = StepIterator::new(&data, 3); // Шаг 3

        let result: Vec<_> = iter.collect();
        assert_eq!(result, vec![&10, &40, &70]);
    }

    #[test]
    fn test_step_iterator_empty() {
        let data: Vec<i32> = Vec::new();
        let iter = StepIterator::new(&data, 1); // Шаг 1, пустой вектор

        let result: Vec<_> = iter.collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_step_iterator_one_element() {
        let data = vec![42];
        let iter = StepIterator::new(&data, 1); // Шаг 1, один элемент

        let result: Vec<_> = iter.collect();
        assert_eq!(result, vec![&42]);
    }
}
