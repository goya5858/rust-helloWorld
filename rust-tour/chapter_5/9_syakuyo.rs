struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1; //参照渡しされたfの値を変更 => 参照元も変更
    // f への可変な参照はここでドロップ
}

fn main() {
    let mut foo = Foo{x: 42};
    do_something(&mut foo);
    // 関数 do_something で可変な参照はドロップされるため、
    // 別の参照を作ることが可能
    do_something(&mut foo);

    println!("{}", foo.x);
}