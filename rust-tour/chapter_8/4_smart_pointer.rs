use std::ops::Deref;
struct TattleTell<T> {
    value: T,
}


// *や.で TattleTellがderefされたときに実行されるメソッドを定義
impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!( "{} was used!", std::any::type_name::<T>() );//Tのtype_nameを取り出す
        &self.value
    }
}

fn main() {
    let foo = TattleTell {
        value: "secret message",
    };
    println!("{}", foo.len());
}