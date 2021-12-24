fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 15 {
            break "15を発見"
        }
    };
    println!("loopの戻り値: {}", v);
    // loopの戻り値: 15を発見
}