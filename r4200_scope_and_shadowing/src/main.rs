fn main() {
    let long_lived_binding = 1u8;

    let shadowed_binding = 3u8;
    {
        let short_lived_binding = 2u8;
        let shadowed_binding = 5u8;
        println!("shadowed in inner : {}", shadowed_binding);
        println!("short lived: {}", short_lived_binding);
    }
    println!("shadowed in outer: {}", shadowed_binding);
    println!("long lived: {}", long_lived_binding);
}
