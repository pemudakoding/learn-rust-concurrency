fn main() {
    println!("Hello, world!");
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

        for i in 0..=5 {
            println!("Counter: {}", counter);
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
}