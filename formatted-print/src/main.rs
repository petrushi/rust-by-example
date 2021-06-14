fn main() {
    let x = 10u32;
    println!("{}",x);

    println!("{0}, {1}, {0}, {1}", "black", "white");

    println!("{subj} {verb} {obj}",
            obj="lazy dog",
            subj="quick fox",
            verb="jumps over"
            );
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=15);
    println!("{number:>0width$}", number=1, width=20);
    #[allow(dead_code)]
    struct Structure(i32);
    //let y = Structure(3);
    println!("This struct '{}' will print...", 3i32);
    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi)
}
