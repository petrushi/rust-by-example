fn main() {
    let a_binding;

    {
        let x = 2u8;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    another_binding = 1u8;
    println!("another binding: {}", another_binding);
}
