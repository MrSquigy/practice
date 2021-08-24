fn main() {
    println!("{} is an argument", 31);

    println!("{0} is an indexed argument. {0} There it is again!", "^");

    println!(
        "{star} these arguments are named {star} {surprised}",
        star = "✨",
        surprised = "😯"
    );

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // right align (>) number padded with up to width 0's
    // remove the 0 for padding with space
    println!(
        "{number} is padded to the right to {width} digits! {number:0>width$}",
        number = 1,
        width = 6
    );

    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` will print!", Structure(3));

    let pi = 3.141592;

    println!("Pi is roughly {:.3}", pi);
}
