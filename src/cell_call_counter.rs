// Счётчик вызовов
// Реализуй структуру CallCounter, которая считает, сколько раз был вызван метод increment(). Используй Cell<u32>.

use std::cell::Cell;

struct CallCounter {
    inner: Cell<u32>
}

impl CallCounter {
    fn new() -> Self {
        CallCounter { inner: Cell::new(0) }
    }
    pub fn increment(&self) {
        self.inner.set(self.inner.get() + 1);
    }
    pub fn get(&self) -> u32 {
        self.inner.get()
    }
}

struct MyBox<T> {
    inner: Cell<T>
}
impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        MyBox { inner: Cell::new(v) }
    }
    pub fn replace(&self, v: T) -> T {
        self.inner.replace(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_box_replace() {
        let b = MyBox::new(10);
        assert_eq!(b.replace(20), 10);
    }
    
    #[test]
    fn test_use() {
        let counter = CallCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }
}
