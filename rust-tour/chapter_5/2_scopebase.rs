struct Foo {
    x: i32,
}

fn main() {
    let foo_a = Foo {x: 42};
    let foo_b = Foo {x: 13};

    println!("{}", foo_a.x);
    println!("{}", foo_b.x);

    // スコープの実行の終了
    // foo_b はここでドロップ
    // foo_a はここでドロップ
}