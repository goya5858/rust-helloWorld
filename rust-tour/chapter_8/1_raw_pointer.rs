fn main() {
    let a = 42;
    // &a という参照を、immutableなi32のポインタ型に変換( as *const i32 )した後、
    // さらにポインタ型を数値(usize)型に変換( as usize )
    let memory_location = &a as *const i32 as usize;
    println!("data here {}", memory_location);
}