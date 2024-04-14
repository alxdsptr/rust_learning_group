use std::panic::PanicInfo;
use std::process;

pub fn panic_handler(panic_info: &PanicInfo) {
    let message = panic_info.payload().downcast_ref::<&str>();
    println!("Panic: {}", if let Some(msg) = message { msg } else { "" });
    process::exit(0);
}

fn read_a_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
fn parse_string_to_a_number(s: String) -> i32 {
    let parsed = s.trim().parse::<i32>();
    match parsed {
        Ok(num) => {
            if num < 0 || num > 100{
                panic!("Number out of range");
            }else{
                num
            }
        },
        Err(error) => {
            panic!("Parse error");
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(panic_handler));
    loop {
        let input = read_a_string();
        let num = parse_string_to_a_number(input);
        println!("Number: {}", num);
    }
}