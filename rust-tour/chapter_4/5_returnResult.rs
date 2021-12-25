fn do_something_that_might_fail(i: i32) -> Result<f32, String>{
    if i == 42 {
        return Ok(13.0);
    } else {
        return Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("発見 {}", v),
        Err(_e) => {
            return Err(String::from("main で何か問題が起きました"));
        }
    }
}