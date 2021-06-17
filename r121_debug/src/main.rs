#[derive(Debug)]
struct UnPrintable(i32);

#[derive(Debug)]
struct Deep(UnPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}
fn main() {
    let x = UnPrintable(3);
    println!("{:?}", x);
    println!("{:?} months in year", 12); // {} == {:?}
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter) // pretty printing
}
