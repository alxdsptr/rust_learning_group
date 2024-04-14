use std::panic::PanicInfo;
use std::process;

#[derive(Debug)]
enum MyError{
    ParseError(std::num::ParseIntError),
    OutOfRange
}

fn read_a_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
fn parse_string_to_a_number(s: String) ->  Result<i32, MyError> {
    let parsed = s.trim().parse::<i32>();
    match parsed {
        Ok(num) => {
            if num < 0 || num > 100{
                Err(MyError::OutOfRange)
            }else{
                Ok(num)
            }
        },
        Err(error) => {
            Err(MyError::ParseError(error))
        }
    }
}
fn main() -> Result<(), MyError> {
    let n = read_a_string().trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let input = read_a_string();
        match parse_string_to_a_number(input) {
            Ok(num) => println!("Number: {}", num),
            Err(MyError::ParseError(e)) => {
                println!("Parse error: {}", e);
            }
            Err(MyError::OutOfRange) => {
                println!("Number out of range");
            }
        }
    }
    Ok(())
}