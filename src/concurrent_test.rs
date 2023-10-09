// atomic int on multiple threads demo

use std::{
    sync::{Arc, Mutex, RwLock},
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

// not work and bad practice
pub fn will_race_condition_way() {
    let mut num = 0;

    let mut handles = vec![];

    fn plus_num(num: &mut i64) {
        *num += 1;
    }

    for _ in 0..100 {
        let handle = thread::spawn(move || plus_num(&mut num));

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("will_race_condition_way(): {:?}", num);
}

fn rwlock_way() {
    let data = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    for _ in 0..100 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_data = data.write().unwrap();
            *write_data += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("rwlock_way(): {}", *data.read().unwrap());
}

pub fn multitest() {
    atomic_way();
    will_race_condition_way();
    rwlock_way();
}
