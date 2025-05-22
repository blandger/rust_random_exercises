// Счётчик с RefCell
// Создай структуру Counter, внутри которой RefCell<u32>.
// Сделай методы:
//
// inc(&self) — увеличивает значение
//
// get(&self) -> u32 — возвращает текущее значение

use std::cell::RefCell;

struct Counter {
    inner: RefCell<u32>
}

impl Counter {
    fn new() -> Self {
        Self { inner: RefCell::new(0) }
    }
    fn inc(&self) {
        *self.inner.borrow_mut() += 1;
    }
    fn get(&self) -> u32 {
        *self.inner.borrow()
    }
}

// Logger
// Сделай структуру Logger, которая:
//
// содержит в себе RefCell<Vec<String>>
//
// имеет метод log(&self, msg: &str) — добавляет строку в лог
//
// метод all(&self) -> Vec<String> — возвращает копию всех сообщений
struct Logger {
    inner: RefCell<Vec<String>>
}
impl Logger {
    pub fn new() -> Self {
        Self { inner: RefCell::new(Vec::new()) }
    }
    fn log(&self, msg: impl Into<String>) {
        self.inner.borrow_mut().push(msg.into());
    }
    fn all(&self) -> Vec<String> {
        self.inner.borrow().clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = Logger::new();
        logger.log("first");
        logger.log("second");
        assert_eq!(logger.all(), vec!["first".to_string(), "second".to_string()]);
    }
    #[test]
    fn test_counter() {
        let c = Counter::new();
        c.inc();
        c.inc();
        assert_eq!(c.get(), 2);
    }
}
