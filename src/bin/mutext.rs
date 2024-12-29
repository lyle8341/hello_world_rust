use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        // MutexGuard智能指针实现了drop trait
        let mut num: std::sync::MutexGuard<'_, i32> = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}