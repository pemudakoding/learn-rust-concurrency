use std::thread;

fn main() {
    let current_thread = thread::current();
    println!("{} :Hello, world!", current_thread.name().unwrap());
}

#[cfg(test)]
mod tests {
    use std::sync::MutexGuard;
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
           for i in 0..=5 {
               println!("hi number {} from the spawned thread!", i);
               thread::sleep(Duration::from_secs(1));
           }
        });

        println!("Application finished");
        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let mut handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter: i32 = 0;

            for i in 0..=5 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }

            return counter;
        });

        println!("Waiting handle ");

        let result: Result<i32, _> = handle.join();

        match result {
            Ok(counter) => println!("Counter = {}", counter),
            Err(error) => println!("Error = {:?}", error),
        }

        println!("Application finished");
    }

    fn calculate() -> i32 {
        let mut counter: i32 = 0;
        let current_thread = thread::current();
        
        
        for i in 0..=5 {
            match current_thread.name() { 
                None => {println!("{:?} : Counter : {}", current_thread.id(), i)}
                Some(name) => {println!("{} : Counter: {}", name, counter)}
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }

        counter
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("Result1 = {}", result1);
        println!("Result2 = {}", result2);
        println!("Application finished");
    }

    #[test]
    fn test_parallel() {
        let handle1: JoinHandle<i32> = thread::spawn(|| calculate());
        let handle2: JoinHandle<i32> = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(result) => println!("Result1 = {}", result),
            Err(error) => println!("Error = {:?}", error),
        }

        match result2 {
            Ok(result) => println!("Result2 = {}", result),
            Err(error) => println!("Error = {:?}", error),
        }


        println!("Application finished");
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        
        println!("Current thread: {}", current_thread.name().unwrap());
        
        let name = String::from("Eko");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello, {}!", name);
        };
        
        let handler = thread::spawn(closure);
        handler.join().unwrap();
    }
    
    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());
        
        let handle = factory
            .spawn(calculate)
            .expect("Failed to create thread");
        
        let total = handle.join().unwrap();
        
        println!("Final result: {}", total);
        
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send(String::from("Hello, World!")).unwrap();
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send(String::from("Hello, World!")).unwrap();
            }

            sender.send("Exit".to_string()).unwrap();
        });

        let handle2 = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();

            if message == "Exit" {
                break;
            }

            println!("{}", message);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send(String::from("Hello, World!")).unwrap();
            }
        });

        let handle2 = thread::spawn(move || {
           for value in receiver.iter() {
               println!("{}", value);
           }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handle3 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(1));
                sender2.send(String::from("Hello, World! sender 2")).unwrap();
            }
        });

        let handle1 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send(String::from("Hello, World! sender 1")).unwrap();
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }

    static mut COUNTER: i32 = 0;

    #[test]
    fn test_race_condition() {
        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || unsafe {
                for i in 0..1000000 {
                    COUNTER += 1;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("COUNTER : {}", unsafe {COUNTER});
    }

    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI32, Ordering};
        
        static counter: AtomicI32 = AtomicI32::new(0);
        
        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || {
                for i in 0..1000000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("COUNTER : {}", counter.load(Ordering::Relaxed));
    }

    #[test]
    fn test_atomic_reference() {
        use std::sync::{
            atomic::{AtomicI32, Ordering}, 
            Arc
        };

        let counter: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
        let mut handles: Vec<JoinHandle<()>> = vec![];
        
        for i in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for i in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("COUNTER : {}", counter.load(Ordering::Relaxed));
    }

    #[test]
    fn test_mutex() {
        use std::sync::{
            Arc,
            Mutex
        };

        let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for i in 0..1000000 {
                   let mut data: MutexGuard<i32> = counter_clone.lock().unwrap();
                    *data += 1;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("COUNTER : {}", *counter.lock().unwrap());
    }
}