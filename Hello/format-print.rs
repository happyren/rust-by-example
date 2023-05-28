fn main() {
    println!("{} days", 31);

    println!{"{0}, this is {1}. {1}, this is {0}.", "Alice","Bob"};

    // Named arguments
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
    
    // Special formatting
    println!("Base 10:          {}", 69420);
    println!("Binary:           {:b}", 69420);
    println!("Hex:              {:x}", 69420);
    println!("HEX:              {:X}", 69420);
    println!("Octal:            {:o}", 69420);
    println!("Scientific:       {:e}", 69420.0);

    // right justify text with a specified width :O
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:>width$}", number=1, width=6);

    // pad number with extra zeros
    println!("{number:0<length$}", number=1, length=6);

    // rust will check correct numbers of args are used.
    // if missing an arg it will say error: invalid reference to positional argument 1 (there is 1 argument)
    println!("The name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // can also capture arguments from a surrounding variable.
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:0>width$}");

    /*
     * Activities
     */
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");
}