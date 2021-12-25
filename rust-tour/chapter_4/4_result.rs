//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

fn do_something_that_might_fail(i: i32) -> Result<f32, String>{
    if i == 42 {
        return Ok(13.0);
    } else {
        return Err(String::from("正しい値ではありません"))
    }
}

fn main() {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v)  => println!("detect {}", v),
        Err(e) => println!("error {}", e)
    }
}