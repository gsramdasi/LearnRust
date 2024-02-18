// Common Notes:
//
// What weare building here is a binary crate. There are also linrary crates.
// rand is one such library crate. To use it, we need to first add it to the
// cargo file and run cargo build.
//
// Cargo first fetches dependencies, then builds them and then this code. While
// fetching and building dependencies, cargo will also get any dependencies that
// these dependencies may have.
// If you run cargo build again, it will be a no-op.
// If you modify src/main.rs and run cargo build, it will only rebuild our change,
// not dependecies.
//
// Cargo.lock
// If rand was added as a dep to Cargo.toml with version 0.8.5, 0.8 is the major
// version and the API isn't expected to change until 0.9.0.
// When we run cargo build for the first time, Cargo.lock records that we got
// version 0.8.5 for rand. Now, even if 0.8.6 is available, Cargo.lock will know
// that 0.8.5 was used last, so it won't get the 0.8.6 build. We can update by
// running cargo update. Not that this will not update to 0.9.0 even if it's
// available.
//
// Running cargo doc --open will build documentation provided by all deps and open
// in the browser.

// Brings in io library from the std library in to scope
use std::cmp::Ordering;
use std::io; // an enum with variants Less, Greater and Equal

// Rng is a trait that defines methods that random number generators (rng) use.
use rand::Rng;

fn main() {
    println!("Guess number");

    // 1..=100 means [1, 100]. The '=' sign makes the upper bound inclusive
    // thread_rng() returns the specific rng we're going to use - rng local to
    // thread.
    // My guess is that what is returned is something that implements the
    // Rng trait.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {secret_number}");

    // loop {} creates an infinite loop
    loop {
        println!("Enter num: ");

        // In Rust, variables are immutable by default. Hence mut keyword.
        // The :: syntax means new is an associated fn of the String type.
        let mut guess = String::new();

        // io::stdin(): returns an instance of std::io::Stdin. which is a
        // type that represents a handle to the standard input for your terminal
        // read_line(): references are also immutable by default. Hence &mut guess
        // and not &guess read_line() returns a Result enum. Its variants are Ok an Err.
        // expect(): Values of any type have methods defined on them. Result is a type
        // and expect() is defined on it.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing the guess variable
        // specifying the u32 type for guess tell parse() which type to convert to.
        // We could use expect() after parse() like before:
        //    let guess: u32 = guess.trim().parse().expect("Please enter a number");
        // but let's be more graceful this time around.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => {
                println!("Too big");
            },
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
