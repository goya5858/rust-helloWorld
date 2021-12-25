//do_something_that_might_fail()?
//
//match do_something_that_might_fail() {
//    Ok(v) => v,
//    Err(e) => return Err(e),
//}
// 上の2式は等価

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {
    let v = do_something_that_might_fail(42)?;
    println!("detect {}", v);
    Ok(())
}