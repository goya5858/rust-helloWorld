struct Foo {
    x: i32,
}

// 'aはライフタイム指定子
// どの引数と戻り値がライフタイムを共有しているかを明示化
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo = Foo{ x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // xはここでdrop
    let y = do_something(&foo);
    println!("{}", y);
    // y はここでドロップ
    // foo はここでドロップ
}
