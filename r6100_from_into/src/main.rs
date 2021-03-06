use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
fn main() {
    let my_str = "hello";
    #[allow(unused_variables)]
    let my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    println!("My number is {:?}", int);
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
