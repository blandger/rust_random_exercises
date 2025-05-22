#![allow(dead_code)]
use std::fmt::{Debug};

use std::ops::{Deref, DerefMut};

// Напиши обёртку, которая может логировать любые действия над объектом
// Условие:
// Представь, что у тебя есть некий тип T, и ты хочешь обернуть его в Logger<T>, который:
//
// реализует std::ops::Deref и DerefMut к T,
//
// при каждом вызове deref() или deref_mut() выводит сообщение в лог (через println!),
//
// должен работать с любым типом T.
struct Logger<T> {
    value: T,
}
impl<T> Logger<T> {
    pub fn new(value: T) -> Self {
        Logger { value }
    }
    pub fn get_ref(&self) -> &T {
        &self.value
    }
    pub fn _get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<T> Deref for Logger<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("Accessing deref...");
        &self.value
    }
}
impl<T: Debug> DerefMut for Logger<T> {
// impl<T: Display > DerefMut for Logger<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // println!("Accessing deref mut with {}...", self.value);
        println!("Accessing deref mut with {:?}...", self.value);
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_one() {
        let mut logged_vec = Logger::new(vec![1, 2, 3]);
        logged_vec.push(4); // должно напечатать лог при обращении
    }
    #[test]
    fn test_two() {
        let mut logged_vec = Logger::new(vec!['1', '2', '3']);
        logged_vec.push('4'); // должно напечатать лог при обращении
    }

    #[test]
    fn test_logger_insert() {
        let map: HashMap<String, usize> = HashMap::new();
        let mut logger = Logger::new(map);

        logger.insert("key1".to_string(), 100);
        logger.insert("key2".to_string(), 200);

        assert_eq!(logger.get_ref().get("key1"), Some(&100));
        assert_eq!(logger.get_ref().get("key2"), Some(&200));
    }
}
