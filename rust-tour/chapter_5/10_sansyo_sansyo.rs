struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo{x: 42};
    let x = &mut foo.x;
    *x = 13; // &foo.x <=> foo.xの参照先を13に変換
    // x はここでドロップされるため、fooの不変な参照が作成可能

    let y = do_something(&foo);
    println!("{}", y);
    // y はここでドロップ
    // foo はここでドロップ
}