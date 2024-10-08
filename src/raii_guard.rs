use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

fn main() {
    let system = System::new();

    for i in 0..5 {
        system.spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));

            if i == 2 {
                panic!("task {} failed", i);
            }

            println!("task {}", i)
        });
    }

    println!("Tasks started count: ");
    system.print_task_count();
    
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Tasks finished count: ");
    system.print_task_count();

}

struct System {
    task_count: Arc<AtomicU32>,
}

impl System {
    fn new() -> Self {
        Self {
            task_count: Arc::new(AtomicU32::new(0)),
        }
    }

    fn spawn(&self, f: impl FnOnce() + Send + 'static) {
        self.task_count.fetch_add(1, Ordering::SeqCst);
        
        let counter_guard = DecreaseOnDrop(self.task_count.clone());
        std::thread::spawn(move || {
            let _guard = counter_guard;
            f();
        });
    }

    fn print_task_count(&self) {
        println!("task count: {}", self.task_count.load(Ordering::SeqCst));
    }
}

struct DecreaseOnDrop(Arc<AtomicU32>);

impl Drop for DecreaseOnDrop {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}