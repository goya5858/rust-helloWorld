fn main() {
    let a: [u8; 4] = [86, 14, 73, 64];

    // メモリのアドレスを単なる数字として扱うのは安全
    let pointer_a = &a as *const u8 as usize;
    println!("data memory location: {}", pointer_a);

    // 数字をf32のraw Pointerに変換すること自体も安全である
    let pointer_b = pointer_a as *const f32;

    let b = unsafe {
        // pointer_bは数字のポインターではあるが、
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);
}   