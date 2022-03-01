use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let counter1 = Arc::new(Mutex::new(1));
    let counter2 = Arc::new(Mutex::new(2));
    let mut handles = vec![];

    
{    let counter1 = Arc::clone(&counter1);
    let counter2 = Arc::clone(&counter2);

    let handle = thread::spawn(move || {
        let num = counter2.lock().unwrap();
        println!("thread 1 locked 2 ({})", num);

        sleep(Duration::from_millis(1000));

        let mut num = counter1.lock().unwrap();
        println!("thread 1 locked 1");


        *num += 1;
    });
    handles.push(handle);}
{
    let counter1 = Arc::clone(&counter1);
    let counter2 = Arc::clone(&counter2);
    let handle = thread::spawn(move || {
        let num = counter1.lock().unwrap();
        println!("thread 2 locked 1 ({})", num);

        sleep(Duration::from_millis(1000));

        let mut num = counter2.lock().unwrap();
        println!("thread 2 locked 2");

        *num += 1;
    });
    handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}