struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo{x: 42}
    //所有権をreturn してる
}

fn main() {
    let foo = do_something();
    println!("{}", foo.x);
}