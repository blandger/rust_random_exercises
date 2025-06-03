/*
Implementing an async worker pool that executes tasks concurrently using Tokio.
The pool should spawn N worker threads.
We want to implement a method submit_task(task: fn() -> ()) that executes a function in a worker.
Can use tokio::sync::mpsc to queue tasks.
and tokio::task::spawn for execution.
*/
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};

type UniversalTask = Box<dyn FnOnce() + Send + 'static>;

struct WorkerPool {
    // Implement storage
    sender: mpsc::Sender<UniversalTask>,
}

impl WorkerPool {
    async fn new(worker_count: usize) -> Self {
        // Initialize pool
        let (sender, receiver) = mpsc::channel::<UniversalTask>(worker_count);
        let receiver = Arc::new(Mutex::new(receiver));

        // start all worker threads
        for i in 0..worker_count {
            let receiver = Arc::clone(&receiver);
            tokio::spawn(async move {
                loop {
                    let task_opt = {
                        let mut lock = receiver.lock().await;
                        lock.recv().await
                    };

                    if let Some(task) = task_opt {
                        task(); // calling FnOnce
                        // emulate work
                        tokio::time::sleep(Duration::from_secs(i as u64)).await;
                    } else {
                        break;
                    }
                }
            });
        }

        Self {
            sender,
        }
    }

    async fn submit_task<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static {
        let _ = self.sender.send(Box::new(task)).await;
    }
}

#[tokio::main]
async fn main() {
    let pool = WorkerPool::new(4).await;
    pool.submit_task(Box::new(|| println!("Task 1 executed"))).await;
    pool.submit_task(Box::new(|| println!("Task 2 executed"))).await;
    pool.submit_task(Box::new(|| println!("Task 3 executed"))).await;
    pool.submit_task(Box::new(|| println!("Task 4 executed"))).await;
    pool.submit_task(Box::new(|| println!("Task 5 executed"))).await;
    pool.submit_task(Box::new(|| println!("Task 6 executed"))).await;
}
