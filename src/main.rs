use std::thread;

fn main() {
    let current_thread = thread::current();
    println!("{} :Hello, world!", current_thread.name().unwrap());
}

#[cfg(test)]
mod tests {
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
}