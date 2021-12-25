fn main() {
    let mut foo = 42;
    let f   = &mut foo; //参照渡し的な 
    let bar = *f; //値渡し的な
    *f = 13; // fooの参照先の値を変更

    println!("{}", bar);
    println!("{}", foo);
}