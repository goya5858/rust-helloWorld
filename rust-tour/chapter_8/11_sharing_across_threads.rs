// 単一のCPUスレッドのみでしか同時にBorrowできない
use::sync::Mutex;

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("only I eat the pie right now!");
    }
}

fn main() {
    let mutex_pie = Mutex::new(Pie); //borrow a locked immutable reference of pie

    let ref_pie = mutex_pie.lock().unwrap(); // we have to unwrap the result of a lock
    ref_pie.eat();
    // locked reference drops here, and mutex protected value can be used by someone else
}