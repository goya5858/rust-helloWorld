struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // fはここでdrop
}

fn main() {
    let foo = Foo{x: 42};
    do_something(foo); //fooの所有権はdo_somethingに移動する
    
    //println!("{}", foo.x); // fooは使えなくなる
}