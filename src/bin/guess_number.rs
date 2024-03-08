//! Implement a guessing number program.
use std::{
    cmp::Ordering,
    io::{self, BufRead},
};
use std::io::stdin;

/// ### Program
///
/// ```ignore
/// use std::{cmp::Ordering, io::{self, BufRead}};
///
/// fn main() {
///     let secret_number: i32 = /* FILL HERE! */; // Don't let others know this!
///
///     println!("Guess the number!");
///
///     println!("Please input your guess. Type `exit` to exit.");
///     
///     let mut buffer = String::new();
///     // TODO: initialize stdin here!
///     /* FILL HERE */
///
///     loop {
///         // For every time you guess, type in the number
///         // TODO: do stdin and remember to clear the buffer first!
///         /* FILL HERE */
///
///         // What is the type of `trimed_buffer`?
///         let trimed_buffer = buffer.trim();
///
///         match trimed_buffer {
///             // TODO: handle `exit` here.
///             /* FILL HERE */
///             trimed_buffer => {
///                 let guess = trimed_buffer.parse::<i32>(); // See what does `parse` do?
///                 match guess {
///                     Ok(num) => // TODO: Guessing number!
///                     Err(_) => // TODO: Something unhappy happened! Handle it!
///                 }
///             }
///         }
///     }
/// }
/// ```
fn main() {
    let secret_number: i32 = 128;

    println!("Guess the number!");

    println!("Please input your guess. Type `exit` to exit.");

    let mut buffer = String::new();
    let mut handle = stdin().lock();
    // TODO: initialize stdin here!
    /* FILL HERE */

    loop {
        // For every time you guess, type in the number
        // TODO: do stdin and remember to clear the buffer first!
        /* FILL HERE */
        buffer.clear();
        handle.read_line(&mut buffer).unwrap();

        // What is the type of `trimed_buffer`?
        let trimed_buffer = buffer.trim();

        match trimed_buffer {
            // TODO: handle `exit` here.
            /* FILL HERE */
            "exit" => break,
            trimed_buffer => {
                let guess = trimed_buffer.parse::<i32>(); // See what does `parse` do?
                match guess {
                    Ok(num) => {
                        if num > secret_number{
                            println!("Greater");
                        }else if num == secret_number{
                            println!("You win!!!");
                            break;
                        }else{
                            println!("Less");
                        }
                    },
                    Err(_) => {
                        println!("what did you do? input the number again");
                    }
                }
            }
        }
    }
}
