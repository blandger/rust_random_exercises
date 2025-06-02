// Счётчик вызовов с Interior Mutability
//
// Напиши структуру CallCounter, которая оборачивает любую замыкаемую функцию и считает, сколько раз её вызывали. Вызов должен быть через метод call(...).

use std::cell::RefCell;

struct CallCounter<F, T, U> where F: Fn(T) -> U {
    counter: RefCell<usize>,
    func: F,
    _marker: std::marker::PhantomData<T>,
}
impl<F, T, U> CallCounter<F, T, U> where  F: Fn(T) -> U {
    pub fn new(func: F) -> Self {
        Self {
            counter: RefCell::new(0),
            func,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn call(&self, value: T) -> U {
        let call_result = (self.func)(value);
        *self.counter.borrow_mut() += 1;
        call_result

    }
    pub fn get_count(&self) -> usize {
        self.counter.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_call_counter_1() {
        let func = |x: i32| x * 2;
        let counter = CallCounter::new(func);

        assert_eq!(counter.call(5), 10);
        assert_eq!(counter.call(2), 4);
        assert_eq!(counter.call(3), 6);
        assert_eq!(counter.get_count(), 3);
    }
    #[test]
    fn test_call_counter_2() {
        let func = |x: i32| x - 2;
        let counter = CallCounter::new(func);

        assert_eq!(counter.call(5), 3);
        assert_eq!(counter.call(2), 0);
        assert_eq!(counter.call(-3), -5);
        assert_eq!(counter.get_count(), 3);
    }
}
