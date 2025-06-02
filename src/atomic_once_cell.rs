// Реализуй тип AtomicOnceCell<T>, который позволяет одному потоку установить значение, а любое число других потоков смогут его безопасно читать.
//
// Требования:
//
// Метод set_once(&self, value: T) -> bool:
//
// устанавливает значение только один раз,
//
// возвращает true, если установка прошла успешно, и false, если значение уже установлено.
//
// Метод get(&self) -> Option<&T>:
//
// возвращает Some(&T) если значение установлено, иначе None.
//
// 📌 Ограничения:
// Тип T: Send + Sync + 'static
//
// Используй только AtomicPtr и безопасную обёртку вокруг него (вроде твоей AtomicOption<T>)
//
// Без блокировок (Mutex, RwLock и пр.)

use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicPtr, Ordering};

struct AtomicOnceCell<T: Send + Sync> {
    ptr: AtomicPtr<T>,
}
impl<T: Send + Sync> AtomicOnceCell<T> {
    pub fn new() -> AtomicOnceCell<T> {
        AtomicOnceCell {
            ptr: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
    pub fn set_once(&self, value: T) -> bool {
        let boxed = ManuallyDrop::new(Box::new(value));
        let raw = Box::into_raw(ManuallyDrop::into_inner(boxed));

        let prev = self.ptr.compare_exchange(
            std::ptr::null_mut(),
            raw,
            Ordering::AcqRel,
            Ordering::Acquire,
        );

        if prev.is_ok() {
            true
        } else {
            // отличие от примера AtomicOption
            // НЕ освобождаем память, если установка не удалась
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
    pub fn is_set(&self) -> bool {
        self.ptr.load(Ordering::SeqCst) != std::ptr::null_mut()
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.ptr.load(Ordering::SeqCst) == std::ptr::null_mut() {
            None
        } else {
            let ptr = self.ptr.load(Ordering::Acquire);
            unsafe { ptr.as_mut() }
        }
    }
    pub fn take(self) -> Option<T> {
        let ptr = self.ptr.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(*Box::from_raw(ptr)) }
        }
    }
}
impl<T: Send + Sync> Drop for AtomicOnceCell<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use super::*;

    #[test]
    fn test_one() {
        let cell = Arc::new(AtomicOnceCell::new());

        let writer = {
            let cell = Arc::clone(&cell);
            thread::spawn(move || {
                assert_eq!(false, cell.is_set());
                assert_eq!(cell.set_once(123), true);
                assert_eq!(true, cell.is_set());
                assert_eq!(cell.set_once(456), false); // вторая попытка — игнорируется
            })
        };

        let readers: Vec<_> = (0..4).map(|_| {
            let cell = Arc::clone(&cell);
            thread::spawn(move || {
                while cell.get().is_none() {
                    thread::yield_now();
                }
                assert_eq!(true, cell.is_set());
                assert_eq!(cell.get(), Some(&123));
            })
        }).collect();

        writer.join().unwrap();
        for r in readers {
            r.join().unwrap();
        }
    }

    #[test]
    fn test_take_and_get_mut() {
        let mut cell = AtomicOnceCell::new();
        assert_eq!(cell.set_once(123), true);

        let r = cell.get_mut().unwrap();
        *r += 1;
        assert_eq!(cell.get(), Some(&124));

        assert_eq!(cell.take(), Some(124));
    }
    
    #[test]
    fn test_three() {
        let msg = "Some message for test!";
        example(msg);
    }

    fn example<'a>(s: &'a str) {
        let cell: AtomicOnceCell<&'a str> = AtomicOnceCell::new();
        cell.set_once(s);
        assert_eq!(cell.get(), Some(&"Some message for test!"));
    }
}
