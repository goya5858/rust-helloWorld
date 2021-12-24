fn make_nothing() -> () {
    return ();
}

fn make_nothing2() {
    // 返値無し
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    println!("aの値:{:?}", a);
    println!("bの値:{:?}", b);
}