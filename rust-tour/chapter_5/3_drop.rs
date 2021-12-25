struct Bar {
    x: i32
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo {
        bar: Bar{ x: 42 }
    };

    println!("{}", foo.bar.x);

    //最初にfooがドロップ（構造体自体）
    //次にfoo.barがドロップ(構造体の子要素)
}