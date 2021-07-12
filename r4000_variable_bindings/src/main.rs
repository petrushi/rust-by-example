fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("{}", an_integer);
    println!("An integer: {:?}", copied_integer);
    println!("bool: {:?}", a_boolean);
    println!("unit: {:?}", unit);

    let _unused_var = 3u8;
    let noisy_unused_var = 2u8;
}
