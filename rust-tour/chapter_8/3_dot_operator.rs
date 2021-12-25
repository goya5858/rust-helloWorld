struct Foo {
    value: i32,
}

fn main() {
    let f = Foo {value: 42};
    let ref_ref_ref_f = &&&f;
    // . operationは自動的にdereferenceをするため、
    // ***ref_ref_ref_fとする必要はない(してはいけない)
    println!("{}", ref_ref_ref_f.value);
    // ref_ref_ref_f.value == (***ref_ref_ref_f).value
}