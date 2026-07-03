#![allow(dead_code)]
/// Пример работы двух потоков и нотификации со стороны продьюсера в поток консьюмера о появлении новых данных в очереди.


/// Common data shared between two threads exchanging values via internal queue
struct SharedState {
    /// shared data in vec
    queue: Vec<i32>,
    /// true - if data is over
    finished: bool,
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;
    use crate::producer_consumer_condvar::SharedState;

    #[test]
    fn test_how_it_works() {
        // The more complex component includes Condvar for notification another thread
        let conditional_shared_state = Arc::new((
            Mutex::new(SharedState {
                queue: Vec::new(),
                finished: false,
            }),
            // That 'Condvar' component will be used for consumer thread notification about new value inside queue
            Condvar::new(),
        ));

        let shared_producer = Arc::clone(&conditional_shared_state);
        let shared_consumer = Arc::clone(&conditional_shared_state);

        // fill in data by Producer thread
        let producer_handle = thread::spawn(move || {
            let (lock, cvar) = &*shared_producer;
            for i in 0..15 {
                let mut state = lock.lock().unwrap();
                println!("Producer is added data: {}", i);
                state.queue.push(i);
                // уведомляем консьюмера после каждого добавления
                cvar.notify_one();
                drop(state); // освобождаем лок
                thread::sleep(Duration::from_millis(500));
            }

            let mut state = lock.lock().unwrap();
            state.finished = true; // filling has been finished
            cvar.notify_one(); // убеждаемся что флаг завершения установлен
        });


        // Run consumer thread then
        let consumer_handle = thread::spawn(move || {
            let (lock, cvar) = &*shared_consumer;
            loop {
                let mut state = lock.lock().unwrap();
                // ждем пока очередь пуста, а флаг завершения - false
                while state.queue.is_empty() && !state.finished {
                    state = cvar.wait(state).unwrap();
                }

                // check завершение loop цикла
                if state.queue.is_empty() && state.finished {
                    break;
                }

                // обработчик полученного значения
                if let Some(value) = state.queue.pop() {
                    println!("Consumer got: {}", value);
                }
            }
        });

        // finish main thread
        producer_handle.join().unwrap();
        consumer_handle.join().unwrap();
        println!("Test End");
    }
}