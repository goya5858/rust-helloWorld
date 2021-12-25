fn main() {
    let a: &'static str = r#"
        <div class="advice">
            raw code
        </div>
        "#;
    println!("{}", a);
}