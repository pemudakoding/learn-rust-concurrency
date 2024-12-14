fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::thread;
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

}