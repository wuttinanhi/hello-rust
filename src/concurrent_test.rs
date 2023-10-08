// atomic int on multiple threads demo

use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn atomic_way() {
    let num = Arc::new(Mutex::new(0 as i64));
    let mut handles = vec![];

    for _ in 0..100 {
        let num_clone = num.clone();
        let handle = thread::spawn(move || {
            let mut num = num_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("atomic_way(): {:?}", *num.lock().unwrap());
}

pub fn race_condition_way() {
    let mut num = 0;
    let mut handles = vec![];

    for _ in 0..100 {
        let handle = thread::spawn(move || {
            num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The value of the integer variable is: {:?}", num);
}

pub fn multitest() {
    atomic_way();
    race_condition_way();
}
