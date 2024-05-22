use std::{
    panic::PanicInfo,
    sync::{Arc, Mutex},
    thread,
};

fn panic_handler(panic_info: &PanicInfo) {
    println!("panic, {}", panic_info);
}

fn main() {
    std::panic::set_hook(Box::new(panic_handler));

    let shared_data = Arc::new(Mutex::new(0));

    // poison the mutex
    let c_mutex = Arc::clone(&shared_data);
    let _ = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || {
            // 导致lock进入poison状态
            // let mut data = c_mutex.lock().unwrap();
            // *data += 1;

            // 方法一:保证不会发生poison的方法1,主动drop Guard
            let mut data = c_mutex.lock().unwrap();
            *data += 1;
            drop(data);

            // 方法二:使用临时变量,Guard跟随临时变量自动释放
            // *c_mutex.lock().unwrap() += 1;

            panic!("panic");
        })
        .unwrap();

    let mut jds = vec![];
    for _ in 0..100 {
        let shared_data = Arc::clone(&shared_data);
        let jd = thread::spawn(move || {
            // PoisonError happen because panic in thread1
            let mut data = shared_data.lock().unwrap();
            *data += 1;
        });
        jds.push(jd);
    }

    for jd in jds {
        jd.join().unwrap();
    }

    println!("result: {:?}", shared_data);
}
