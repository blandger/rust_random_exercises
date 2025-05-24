#[allow(dead_code)]

// Сделать структуру AtomicOption<T>, которая использует AtomicPtr<T> и позволяет однократно установить значение (set_once()), и потом безопасно его читать (get()), без блокировок.

use std::sync::atomic::{AtomicPtr, Ordering};

struct AtomicOption<T> {
    ptr: AtomicPtr<T>,
}
impl<T> AtomicOption<T> {
    pub fn new() -> AtomicOption<T> {
        AtomicOption {
            ptr: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
    pub fn set_once(&self, value: T) -> bool {
        let boxed = Box::new(value);
        let raw = Box::into_raw(boxed);

        let prev = self.ptr.compare_exchange(
            std::ptr::null_mut(),
            raw,
            Ordering::AcqRel,
            Ordering::Acquire,
        );

        if prev.is_ok() {
            true
        } else {
            // Освобождаем память, если установка не удалась
            unsafe {
                drop(Box::from_raw(raw));
            }
            false
        }
    }
    pub fn get(&self) -> Option<&T> {
        if self.ptr.load(Ordering::SeqCst) == std::ptr::null_mut() {
            None
        } else {
            let ptr = self.ptr.load(Ordering::Acquire);
            unsafe { ptr.as_ref() }
        }
    }
}

impl<T> Drop for AtomicOption<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Acquire);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering as AtomicOrdering}};
    use super::*;

    #[test]
    fn test_atomic_option_set_and_get() {
        let atomic = AtomicOption::new();
        // set_once должен вернуть true при первом вызове
        assert!(atomic.set_once(123));
        // Повторная установка — false
        assert!(!atomic.set_once(456));
        // Проверяем значение
        assert_eq!(atomic.get(), Some(&123));
    }

    #[test]
    fn test_drop_cleans_up_memory() {
        struct Tracker {
            drops: Arc<AtomicUsize>,
        }

        impl Drop for Tracker {
            fn drop(&mut self) {
                self.drops.fetch_add(1, AtomicOrdering::SeqCst);
            }
        }

        let counter = Arc::new(AtomicUsize::new(0));
        let tracker = Tracker { drops: Arc::clone(&counter) };

        {
            let atomic = AtomicOption::new();
            assert!(atomic.set_once(tracker));
            assert_eq!(counter.load(AtomicOrdering::SeqCst), 0);
            // `atomic` выйдет из области видимости, и должен вызвать drop для Tracker
        }

        // Теперь объект должен быть удалён
        assert_eq!(counter.load(AtomicOrdering::SeqCst), 1);
    }

    use std::sync::Barrier;
    use std::thread;

    #[test]
    fn test_concurrent_reads_after_set_once() {
        const THREADS: usize = 10;

        let atomic = Arc::new(AtomicOption::new());
        let barrier = Arc::new(Barrier::new(THREADS + 1)); // +1 для основного потока

        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                let atomic = Arc::clone(&atomic);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    // Все потоки ждут установки значения
                    barrier.wait();
                    // После барьера читают значение
                    let val = atomic.get();
                    assert_eq!(val.map(|v| *v), Some(123)); // ожидаемое значение
                })
            })
            .collect();

        // Устанавливаем значение до пропуска барьера
        atomic.set_once(123);

        // Разрешаем всем потокам продолжить
        barrier.wait();

        // Ждём завершения всех потоков
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
