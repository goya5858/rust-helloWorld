struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo{x: 42};
    let f = &foo; //アクセスを借用
    println!("{}", f.x);

    // fはここでdrop
    // fooもここでdrop
}