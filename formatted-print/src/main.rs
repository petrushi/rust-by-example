fn main() {
    let x = 10u32;
    println!("{}",x);

    println!("{0}, {1}, {0}, {1}", "black", "white");

    println!("{subj} {verb} {obj}",
            obj="lazy dog",
            subj="quick fox",
            verb="jumps over"
            );
}
