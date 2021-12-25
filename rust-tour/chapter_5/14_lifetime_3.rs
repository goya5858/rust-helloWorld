 // Rust は、参照を含む構造体が、
 // その参照が指す所有者よりも長く存在しないことを検証します。
 
 struct Foo<'a> {
     i: &'a i32,
 }

 fn main() {
     let x = 42;
     let foo = Foo{
         i: &x,
     };
     println!("{}", foo.i);
 }