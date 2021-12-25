// データ競合を防止するため、Rust では同時に 2 つの変数から値を変更することはできません。

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
}

fn main() {
    let mut foo = Foo{x: 32};
    let f = &mut foo;
    // ここでは foo は f へ可変に借用されており
    // do_something(foo);
    // で、移動できない

    //foo.x = 13; // も、 foo が f へ可変に借用されているためエラー
    f.x = 13;
    // f はここから先では使用されないため、ここでドロップ
    println!("{}", foo.x);
    
    // 可変な借用はドロップされているため変更可能
    foo.x = 7;
    do_something(foo);
}